# TODO:
#   - Import a lib (for save time)
#   - Create a valid raw transaction
#   - Split the transaction
#   - Replace transaction parameters by misfit parameters 
#   - Assemble misfit transaction 

from bcli import bcli
import json
class CreateTx:
    def __init__(self, **args):
        # Transaction
        self.tx_count: int = args.get("tx_count", 0)
        self.invalid_tx_count: int = args.get("invalid_tx_count", 0)

        self.tx_version: bool = args.get("tx_version", False)
        self.tx_marker: bool = args.get("tx_marker", False)
        self.tx_flag: bool = args.get("tx_flag", False)
        self.tx_locktime: bool = args.get("tx_locktime", False)

        # Inputs
        self.tx_in_count: int = args.get("tx_in_count", 0)
        self.invalid_tx_in_count: int = args.get("invalid_tx_in_count", 0)

        self.tx_in_txid: bool = args.get("tx_in_txid", False)
        self.tx_in_vout: bool = args.get("tx_in_vout", False)
        self.tx_in_script_size: bool = args.get("tx_in_script_size", False)
        self.tx_in_script: bool = args.get("tx_in_script", False)
        self.tx_in_sequence: bool = args.get("tx_in_sequence", False)
        
        # Outputs
        self.tx_out_count: int = args.get("tx_out_count", 0)
        self.invalid_tx_out_count: int = args.get("invalid_tx_out_count", 0)
        
        self.tx_out_amount: bool = args.get("tx_out_amount", False)
        self.tx_out_script_size: bool = args.get("tx_out_script_size", False)
        self.tx_out_script: bool = args.get("tx_out_script", False)

        # Witness
        self.tx_witness_count: int = args.get("tx_witness_count", 0)
        self.invalid_tx_witness_count: int = args.get("invalid_tx_witness_count", 0)

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
        signed_tx = json.loads(bcli(f'signrawtransactionwithwallet {raw_tx}'))['hex']

        # Check if is valid tx
        print("Checking if is valid transaction")
        json.loads(bcli(f'testmempoolaccept ["{signed_tx}"]'))[0]
        
        return raw_tx

    def split_transaction(raw_tx: str) -> object:
        # Decode transaction
        print("Decode transaction")
        decoded_tx = json.loads(bcli(f'decoderawtransaction {raw_tx}'))
        return decoded_tx

    def replace_misfit():
        # TODO: Replace parameters by misfit params
        return
    
    def assemble_transaction():
        # TODO: Assemble misfit transaction
        return

#🐟🏐🐈