import struct
import hashlib
import binascii

class CreateTx:
    def __init__(self, **args):
        # Set class variables by arguments
        self.tx_count = args.get('tx_count', 0)
        self.invalid_tx_count = args.get('invalid_tx_count', 0)
        self.tx_version = args.get('tx_version', False)
        self.tx_marker = args.get('tx_marker', False)
        self.tx_flag = args.get('tx_flag', False)
        self.tx_locktime = args.get('tx_locktime', False)
        
        # Inputs
        self.tx_in_count = args.get('tx_in_count', 0)
        self.invalid_tx_in_count = args.get('invalid_tx_in_count', 0)
        self.tx_in_txid = args.get('tx_in_txid', False)
        self.tx_in_vout = args.get('tx_in_vout', False)
        self.tx_in_script_size = args.get('tx_in_script_size', 0)
        self.tx_in_script = args.get('tx_in_script', False)
        self.tx_in_sequence = args.get('tx_in_sequence', False)
        
        # Outputs
        self.tx_out_count = args.get('tx_out_count', 0)
        self.invalid_tx_out_count = args.get('invalid_tx_out_count', 0)
        self.tx_out_amount = args.get('tx_out_amount', 0)
        self.tx_out_script_size = args.get('tx_out_script_size', 0)
        self.tx_out_script = args.get('tx_out_script', False)

        # Witness
        self.tx_witness_count = args.get('tx_witness_count', 0)
        self.invalid_tx_witness_count = args.get('invalid_tx_witness_count', 0)
        self.tx_witness_size = args.get('tx_witness_size', 0)
        self.tx_witness_item = args.get('tx_witness_item', False)

    def create(self):
        # Create a valid raw transaction
        tx = struct.pack('<I', self.tx_version)
        
        if self.tx_marker:
            tx += b'\x00'
        if self.tx_flag:
            tx += b'\x01'
        
        tx += struct.pack('<B', self.tx_in_count)
        
        for _ in range(self.tx_in_count):
            tx += binascii.unhexlify(self.tx_in_txid)[::-1]
            tx += struct.pack('<I', self.tx_in_vout)
            tx += struct.pack('<B', self.tx_in_script_size)
            tx += self.tx_in_script
            tx += struct.pack('<I', self.tx_in_sequence)
        
        tx += struct.pack('<B', self.tx_out_count)
        
        for _ in range(self.tx_out_count):
            tx += struct.pack('<Q', self.tx_out_amount)
            tx += struct.pack('<B', self.tx_out_script_size)
            tx += self.tx_out_script
        
        if self.tx_witness_count > 0:
            for _ in range(self.tx_witness_count):
                tx += struct.pack('<B', self.tx_witness_size)
                tx += self.tx_witness_item
        
        tx += struct.pack('<I', self.tx_locktime)
        
        return tx

    def split(self, raw_tx):
        # Split raw transaction into components
        offset = 0
        version = struct.unpack('<I', raw_tx[offset:offset+4])[0]
        offset += 4
        
        marker = None
        flag = None
        if raw_tx[offset] == 0:
            marker = raw_tx[offset]
            offset += 1
            flag = raw_tx[offset]
            offset += 1
        
        tx_in_count = raw_tx[offset]
        offset += 1
        
        inputs = []
        for _ in range(tx_in_count):
            txid = raw_tx[offset:offset+32][::-1]
            offset += 32
            vout = struct.unpack('<I', raw_tx[offset:offset+4])[0]
            offset += 4
            script_size = raw_tx[offset]
            offset += 1
            script = raw_tx[offset:offset+script_size]
            offset += script_size
            sequence = struct.unpack('<I', raw_tx[offset:offset+4])[0]
            offset += 4
            inputs.append((txid, vout, script_size, script, sequence))
        
        tx_out_count = raw_tx[offset]
        offset += 1
        
        outputs = []
        for _ in range(tx_out_count):
            amount = struct.unpack('<Q', raw_tx[offset:offset+8])[0]
            offset += 8
            script_size = raw_tx[offset]
            offset += 1
            script = raw_tx[offset:offset+script_size]
            offset += script_size
            outputs.append((amount, script_size, script))
        
        witnesses = []
        if marker is not None:
            for _ in range(tx_in_count):
                witness_size = raw_tx[offset]
                offset += 1
                witness_item = raw_tx[offset:offset+witness_size]
                offset += witness_size
                witnesses.append((witness_size, witness_item))
        
        locktime = struct.unpack('<I', raw_tx[offset:offset+4])[0]
        offset += 4
        
        return {
            'version': version,
            'marker': marker,
            'flag': flag,
            'tx_in_count': tx_in_count,
            'inputs': inputs,
            'tx_out_count': tx_out_count,
            'outputs': outputs,
            'witnesses': witnesses,
            'locktime': locktime
        }

    def replace(self, tx_components):
        # Replace parameters by misfit params
        tx_components['version'] = 0xffffffff  # Invalid version
        tx_components['inputs'][0] = (b'\x00'*32, 0xffffffff, 0, b'', 0xffffffff)  # Invalid input
        tx_components['outputs'][0] = (0xffffffffffffffff, 0, b'')  # Invalid output
        return tx_components

    def assemble(self, tx_components):
        # Assemble misfit transaction
        tx = struct.pack('<I', tx_components['version'])
        
        if tx_components['marker'] is not None:
            tx += struct.pack('<B', tx_components['marker'])
            tx += struct.pack('<B', tx_components['flag'])
        
        tx += struct.pack('<B', tx_components['tx_in_count'])
        
        for txid, vout, script_size, script, sequence in tx_components['inputs']:
            tx += txid[::-1]
            tx += struct.pack('<I', vout)
            tx += struct.pack('<B', script_size)
            tx += script
            tx += struct.pack('<I', sequence)
        
        tx += struct.pack('<B', tx_components['tx_out_count'])
        
        for amount, script_size, script in tx_components['outputs']:
            tx += struct.pack('<Q', amount)
            tx += struct.pack('<B', script_size)
            tx += script
        
        if tx_components['witnesses']:
            for witness_size, witness_item in tx_components['witnesses']:
                tx += struct.pack('<B', witness_size)
                tx += witness_item
        
        tx += struct.pack('<I', tx_components['locktime'])
        
        return tx

# Example usage:
tx_creator = CreateTx(
    tx_count=1,
    tx_version=1,
    tx_in_count=1,
    tx_in_txid='0000000000000000000000000000000000000000000000000000000000000000',
    tx_in_vout=0,
    tx_in_script_size=0,
    tx_in_script=b'',
    tx_in_sequence=0xffffffff,
    tx_out_count=1,
    tx_out_amount=100000000,
    tx_out_script_size=0,
    tx_out_script=b'',
    tx_witness_count=0,
    tx_locktime=0
)

raw_tx = tx_creator.create()
print("Raw Transaction:", binascii.hexlify(raw_tx))

tx_components = tx_creator.split(raw_tx)
print("Transaction Components:", tx_components)

misfit_tx_components = tx_creator.replace(tx_components)
print("Misfit Transaction Components:", misfit_tx_components)

misfit_raw_tx = tx_creator.assemble(misfit_tx_components)
print("Misfit Raw Transaction:", binascii.hexlify(misfit_raw_tx))