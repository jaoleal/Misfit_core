# TODO:
#   - Import a lib (for save time)
#   - Create a valid raw block
#   - Split the block
#   - Replace block parameters by misfit parameters 
#   - Assemble misfit block

class CreateBlock:
    def __init__ (self, **args):
        self.version: bool = False
        self.prevblock: bool = False
        self.merkleroot: bool = False
        self.timestamp: bool = False
        self.bits: bool = False
        self.nonce: bool = False
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

    def create(self, **args) -> str:
        # TODO: Create a valid raw block

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