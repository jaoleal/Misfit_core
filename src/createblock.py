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

from src.utils import bcli, randomize
import json
import sys


class CreateBlock:
    def __init__(self, **kwargs):
        self.version: bool = kwargs.get("version", False)
        self.prevblock: bool = kwargs.get("prevblock", False)
        self.merkleroot: bool = kwargs.get("merkleroot", False)
        self.timestamp: bool = kwargs.get("timestamp", False)
        self.bits: bool = kwargs.get("bits", False)
        self.nonce: bool = kwargs.get("nonce", False)

        self.tx_count: int = kwargs.get("tx_count", 0)
        self.invalid_tx_count: int = kwargs.get("invalid_tx_count", 0)
        self.tx_version: bool = kwargs.get("tx_version", False)
        self.tx_marker: bool = kwargs.get("tx_marker", False)
        self.tx_flag: bool = kwargs.get("tx_flag", False)
        self.tx_locktime: bool = kwargs.get("tx_locktime", False)
        self.tx_in_count: int = kwargs.get("tx_in_count", 0)
        self.invalid_tx_in_count: int = kwargs.get("invalid_tx_in_count", 0)
        self.tx_in_txid: bool = kwargs.get("tx_in_txid", False)
        self.tx_in_vout: bool = kwargs.get("tx_in_vout", False)
        self.tx_in_script_size: int = kwargs.get("tx_in_script_size", 0)
        self.tx_in_script: bool = kwargs.get("tx_in_script", False)
        self.tx_in_sequence: bool = kwargs.get("tx_in_sequence", False)
        self.tx_out_count: int = kwargs.get("tx_out_count", 0)
        self.invalid_tx_out_count: int = kwargs.get("invalid_tx_out_count", 0)
        self.tx_out_amount: int = kwargs.get("tx_out_amount", 0)
        self.tx_out_script_size: int = kwargs.get("tx_out_script_size", 0)
        self.tx_out_script: bool = kwargs.get("tx_out_script", False)
        self.tx_witness_count: int = kwargs.get("tx_witness_count", 0)
        self.invalid_tx_witness_count: int = kwargs.get(
            "invalid_tx_witness_count", 0)
        self.tx_witness_size: int = kwargs.get("tx_witness_size", 0)
        self.tx_witness_item: bool = kwargs.get("tx_witness_item", False)

    def create_valid_block():
        return

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
