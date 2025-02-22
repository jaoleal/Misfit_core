# TODO:
#   - Import a lib (for save time)
#   - Create a valid raw block
#   - Split the block
#   - Replace block parameters by misfit parameters 
#   - Assemble misfit block
from utils import merkleroot, reverse_bytes, take_time
class CreateBlock:
    def __init__ (self, **args):
        self.version: bool = False
        self.prevblock: bool = False
        self.merkleroot: bool = False
        self.timestamp: bool = False
        self.bits: bool = False
        self.nonce: bool = False
        self.txs: bool = False
        self.tx_count: int = 0
        self.invalid_tx_count: int = 0
        self.tx_version: bool = False
        self.tx_marker: bool = False
        self.tx_flag: bool = False
        self.tx_locktime: bool = False
        self.tx_in_count: int = 0
        self.invalid_tx_in_count: int = 0
        self.tx_in_txid: bool = False
        self.tx_in_vout: bool = False
        self.tx_in_script_size: int = 0
        self.tx_in_script: bool = False
        self.tx_in_sequence: bool = False
        self.tx_out_count: int = 0
        self.invalid_tx_out_count: int = 0
        self.tx_out_amount: int = 0
        self.tx_out_script_size: int = 0
        self.tx_out_script: bool = False
        self.tx_witness_count: int = 0
        self.invalid_tx_witness_count: int = 0
        self.tx_witness_size: int = 0
        self.tx_witness_item: bool = False
        return

    def create(txs:dict,previous_block:str,version=0,split=False) -> str:
        # TODO: Create a valid raw block
        """In the we will need:
        version 4 bytes litle
        previous block 32 bytes natural bytes order
        merkle root 32 bytes natural bytes order
        time  4 bytes litle endian
        bits 4 bytes litle
        Nonce 4 bytes litle
        transaction count compact size
        transactions all of the raw transactions  """  
        count = 0
        txids = []
        txs_concatened = ""
        for b in txs:
            count += 1
        for e in txs[txids]:
            txids.append(c) 
        merkle_root = reverse_bytes(merkleroot(txids))
        for c in txs:
            txs_concatened += c
        block = version + previous_block + merkle_root + take_time() + self.bits + self.nonce + count + txs_concatened
        return block

    def split():
        # TODO: Split the block
        create()
        return

    def replace():
        # TODO: Replace parameters by misfit params
        return

    def assemble():
        # TODO: Assemble misfit block
        return