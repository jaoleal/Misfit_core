class CreateTx:
    self.tx_count: int = 0
    self.invalid_tx_count: int = 0
    self.tx_version: bool = False
    self.tx_marker: bool = False
    self.tx_flag: bool = False
    self.tx_locktime: bool = False
    
    # Inputs
    self.tx_in_count: int = 0
    self.invalid_tx_in_count: int = 0
    self.tx_in_txid: bool = False
    self.tx_in_vout: bool = False
    self.tx_in_script_size: int = 0
    self.tx_in_script: bool = False
    self.tx_in_sequence: bool = False
    
    # Outputs
    self.tx_out_count: int = 0
    self.invalid_tx_out_count: int = 0
    self.tx_out_amount: int = 0
    self.tx_out_script_size: int = 0
    self.tx_out_script: bool = False

    # Witness
    self.tx_witness_count: int = 0
    self.invalid_tx_witness_count: int = 0
    self.tx_witness_size: int = 0
    self.tx_witness_item: bool = False

    def __init__(self, **args):
        # TODO: Set class variables by arguments
        return

    def create(self):
        # TODO: Create misfit transaction 

    
    @staticmethod
    def new(self, **args):
        # TODO: Generate and returns a default valid transaction
        return

