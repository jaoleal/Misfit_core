# TODO:
#   - Import a lib (for save time)
#   - Create a valid raw block
#   - Split the block
#   - Replace block parameters by misfit parameters
#   - Assemble misfit block

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
        return

    def replace():
        # TODO: Replace parameters by misfit params
        return

    def assemble():
        # TODO: Assemble misfit block
        return
