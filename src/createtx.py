# TODO:
#   - Import a lib (for save time)
#   - Create a valid raw transaction
#   - Split the transaction
#   - Replace transaction parameters by misfit parameters
#   - Assemble misfit transaction

from utils import bcli, randomize
import json


class CreateTx:
    def __init__(self, **args):
        # Transaction
        self.tx_version: bool = args.get("tx_version", False)
        self.tx_locktime: bool = args.get("tx_locktime", False)

        # Inputs
        # TODO: self.tx_in_count: int = args.get("tx_in_count", 0)
        # TODO: self.invalid_tx_in_count: int = args.get("invalid_tx_in_count", 0)

        self.tx_in_txid: bool = args.get("tx_in_txid", False)
        # TODO: self.tx_in_vout: bool = args.get("tx_in_vout", False)
        self.tx_in_script: bool = args.get("tx_in_script", False)
        self.tx_in_sequence: bool = args.get("tx_in_sequence", False)

        # Outputs
        # TODO: self.tx_out_count: int = args.get("tx_out_count", 0)
        # TODO: self.invalid_tx_out_count: int = args.get("invalid_tx_out_count", 0)

        # TODO: self.tx_out_amount: bool = args.get("tx_out_amount", False)
        self.tx_out_script_size: bool = args.get("tx_out_script_size", False)
        self.tx_out_script: bool = args.get("tx_out_script", False)

        # Witness
        # TODO: self.tx_witness_count: int = args.get("tx_witness_count", 0)
        # TODO: self.invalid_tx_witness_count: int = args.get("invalid_tx_witness_count", 0)

        self.tx_witness_size: bool = args.get("tx_witness_size", False)
        self.tx_witness_item: bool = args.get("tx_witness_item", False)

    def create_valid_tx() -> str:
        # Create a wallet
        print("Creating misfit-core wallet")
        try:
            bcli("createwallet misfit-wallet")
        except:
            print("misfit-wallet already exists")

        # Generate new address for wallet
        print("Generating new wallet")
        addr = bcli("getnewaddress")

        # Generate funds to the address
        print("Generating funds to the address")
        blockhash = json.loads(bcli(f"generatetoaddress 101 {addr}"))[0]

        # Get block data
        print("Get block data")
        block = json.loads(bcli(f"getblock {blockhash} 2"))

        # Get transaction from block
        tx = block['tx'][0]

        # Create valid raw transaction
        print("Creating a raw transaction")
        tx_in = json.dumps([{
            "txid": tx['txid'],
            "vout": tx['vout'][0]['n']
        }], separators=(',', ':'))

        amount = tx['vout'][0]['value'] - 0.01

        tx_out = json.dumps([{
            bcli("getnewaddress"): amount
        }], separators=(',', ':'))

        raw_tx = bcli(f'createrawtransaction {tx_in} {tx_out}')

        # Sign transaction
        print("Sign transaction")
        signed_tx = json.loads(
            bcli(f'signrawtransactionwithwallet {raw_tx}'))['hex']

        # Check if is valid tx
        print("Checking if is valid transaction")
        json.loads(bcli(f'testmempoolaccept ["{signed_tx}"]'))[0]

        return raw_tx

    def split_transaction(raw_tx: str) -> object:
        offset = 0
        txns = bytes.fromhex(raw_tx)

        version = txns[offset:offset+4]
        offset += 4

        marker = txns[offset:offset+1]
        offset += 1

        flag = txns[offset:offset+1]
        offset += 1

        tx_in_count = txns[offset:offset+1]
        offset += 1

        tx_in = []
        for _ in range(int.from_bytes(tx_in_count)):
            previous_output = txns[offset:offset+36]
            offset += 36

            script_length = txns[offset:offset+1]
            offset += 1

            signature_script = txns[offset:offset +
                                    int.from_bytes(script_length)]
            offset += int.from_bytes(script_length)

            sequence = txns[offset:offset+4]
            offset += 4

            tx_in.append({
                "previous_output": previous_output.hex(),
                "script_length": script_length.hex(),
                "signature_script": signature_script.hex(),
                "sequence": sequence.hex()
            })

        tx_out_count = txns[offset:offset+1]
        offset += 1

        tx_out = []
        for _ in range(int.from_bytes(tx_out_count)):
            value = txns[offset:offset+8]
            offset += 8

            pk_script_length = txns[offset:offset+1]
            offset += 1

            pk_script = txns[offset:offset+int.from_bytes(pk_script_length)]
            offset += int.from_bytes(pk_script_length)

            tx_out.append({
                "value": value.hex(),
                "pk_script_length": pk_script_length.hex(),
                "pk_script": pk_script.hex()
            })

        witness_count = txns[offset:offset+1]
        offset += 1

        witness = []
        for _ in range(int.from_bytes(witness_count)):
            size = txns[offset:offset+1]
            offset += 1

            item = txns[offset:offset+int.from_bytes(size)]
            offset += int.from_bytes(size)

            witness.append({
                "size": size.hex(),
                "item": item.hex()
            })

        lock_time = txns[offset:offset+4]
        offset += 4

        return {
            "version": version.hex(),
            "marker": marker.hex(),
            "flag": flag.hex(),
            "tx_in_count": tx_in_count.hex(),
            "tx_in": tx_in,
            "tx_out_count": tx_out_count.hex(),
            "tx_out": tx_out,
            "witness": witness,
            "locktime": lock_time.hex()
        }

    def replace_misfit(decoded_tx: object) -> object:
        if self.version:
            decoded_tx['version'] = randomize(decoded_tx['version'])

        if self.locktime:
            decoded_tx['locktime'] = randomize(decoded_tx['locktime'])

        if self.tx_in_txid:
            for tx_in in decoded_tx['tx_in']:
                tx_in = randomize(tx_in['previous_output'])

        return decoded_tx

    def assemble_transaction(decoded_tx: object) -> str:
        version = decoded_tx['version'].to_bytes(4, "little")
        flags = bytes.fromhex("0001")
        locktime = decoded_tx['locktime'].to_bytes(4, "little")

        inputs = []
        for input in decoded_tx['vin']:
            txid = bytes.fromhex(input['txid'])
            vout = input['vout'].to_bytes(4, "little")
            script = bytes.fromhex(input['scriptSig']['hex'])
            scriptlen = bytes([len(script)])
            sequence = input['sequence'].to_bytes(4, 'little')
            inputs.append(txid + vout + scriptlen + script + sequence)

        outputs = []
        for output in decoded_tx['vout']:
            amount = int(output['value'] * 100_000_000).to_bytes(8, 'little')
            scriptpubkey = output['scriptPubKey']['hex']
            scriptpubkeylen = bytes([len(scriptpubkey)])
            outputs.append(amount + scriptpubkeylen + scriptpubkey)

        witnesses = []
        for witness in decoded_tx['vout']:
            amount = int(output['value'] * 100_000_000).to_bytes(8, 'little')
            scriptpubkey = output['scriptPubKey']['hex']
            scriptpubkeylen = bytes([len(scriptpubkey)])
            outputs.append(amount + scriptpubkeylen + scriptpubkey)

        # [version] [flags] [inputs lenght] [inputs] [outputs lenght] [outputs] [witness] [locktime]
        transaction = version + flags + len(inputs).to_bytes() + b''.join(inputs) + len(
            outputs).to_bytes() + b''.join(outputs) + b''.join(witnesses) + locktime
        return transaction.hex()

# {
#   "txid": "0776101a6c0378d142656035f87721aee8e20f8a7c41c3962ad02a3ca8c18a08",
#   "hash": "0776101a6c0378d142656035f87721aee8e20f8a7c41c3962ad02a3ca8c18a08",
#   "version": 2,
#   "size": 82,
#   "vsize": 82,
#   "weight": 328,
#   "locktime": 0,
#   "vin": [
#     {
#       "txid": "d708f91be24be28416a37aa5255173941f597bd442e72293346e62af3c2482c7",
#       "vout": 0,
#       "scriptSig": {
#         "asm": "",
#         "hex": ""
#       },
#       "sequence": 4294967293
#     }
#   ],
#   "vout": [
#     {
#       "value": 24.99,
#       "n": 0,
#       "scriptPubKey": {
#         "asm": "0 98e036c901ea2d31bad07c9b086ec85427a87742",
#         "desc": "addr(bcrt1qnrsrdjgpagknrwks0jdssmkg2sn6sa6z3e7m6v)#mvlsgpnt",
#         "hex": "001498e036c901ea2d31bad07c9b086ec85427a87742",
#         "address": "bcrt1qnrsrdjgpagknrwks0jdssmkg2sn6sa6z3e7m6v",
#         "type": "witness_v0_keyhash"
#       }
#     }
#   ]
# }
