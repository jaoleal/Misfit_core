import argparse
import createtx
import createblock
def main():
    parser = argparse.ArgumentParser(description="A tool for create specified invalid parameters for tests in bicoin.")
    subparsers = parser.add_subparsers(title="Commands", dest="commands")

    # Commands
    parser_createblock = subparsers.add_parser("createblock", help="Create a block with invalid specified parameters")
    parser_createtx = subparsers.add_parser("createtx", help="Create transaction with invalid specified parameters")

    # createblock arguments
    parser_createblock.add_argument('--version', dest="block_version", default=False, type=bool, help="Set the block version as random invalid parameter")
    parser_createblock.add_argument('--prevblock', dest="block_prevblock", default=False, type=bool, help="Set the previous block hash as random invalid parameter")
    parser_createblock.add_argument('--merkleroot', dest="block_merkleroot", default=False, type=bool, help="Set the merkle root as random invalid parameter")
    parser_createblock.add_argument('--timestamp', dest="block_timestamp", default=False, type=bool, help="Set the timestamp as random invalid parameter")
    parser_createblock.add_argument('--bits', dest="block_bits", default=False, type=bool, help="Set the bits as random invalid parameter")
    parser_createblock.add_argument('--nonce', dest="block_nonce", default=False, type=bool, help="Set the nonce as random invalid parameter")
    parser_createblock.add_argument('--tx_count', dest="block_tx_count", default=0, type=int, help="Set a number of transactions in block")
    parser_createblock.add_argument('--invalid_tx_count', dest="block_invalid_tx_count", default=0, type=int, help="Set a number of invalid transactions in block (if is not set, all transactions will be invalid)")
    parser_createblock.add_argument('--tx_version', dest="block_tx_version", default=False, type=bool, help="Set the transaction version as random invalid parameter")
    parser_createblock.add_argument('--tx_marker', dest="block_tx_marker", default=False, type=bool, help="Set the transaction market as random invalid parameter")
    parser_createblock.add_argument('--tx_flag', dest="block_tx_flag", default=False, type=bool, help="Set the transaction flag as random invalid parameter")
    parser_createblock.add_argument('--tx_locktime', dest="block_tx_locktime", default=False, type=bool, help="Set the transaction locktime as random invalid parameter")
    parser_createblock.add_argument('--tx_in_count', dest="block_tx_in_count", default=0, type=int, help="Set the number of transaction inputs")
    parser_createblock.add_argument('--invalid_tx_in_count', dest="block_invalid_tx_in_count", default=0, type=int, help="Set the number of invalid transaction inputs (if not set, all inputs will be invalid)")
    parser_createblock.add_argument('--tx_in_txid', dest="block_tx_in_txid", default=False, type=bool, help="Set the txid from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_vout', dest="block_tx_in_vout", default=False, type=bool, help="Set the vout from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_script_size', dest="block_tx_in_script_size", default=0, type=int, help="Set the size of scriptsig from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_script', dest="block_tx_in_script", default=False, type=bool, help="Set the scriptsig from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_in_sequence', dest="block_tx_in_sequence", default=False, type=bool, help="Set the sequence from inputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_out_count', dest="block_tx_out_count", default=0, type=int, help="Set the number of transaction outputs")
    parser_createblock.add_argument('--invalid_tx_out_count', dest="block_invalid_tx_out_count", default=0, type=int, help="Set the number of invalid transaction outputs (if not set, all outputs will be invalid)")
    parser_createblock.add_argument('--tx_out_amount', dest="block_tx_out_amount", default=0, type=int, help="Set the amount from outputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_out_script_size', dest="block_tx_out_script_size", default=0, type=int, help="Set the size of scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_out_script', dest="block_tx_out_script", default=False, type=bool, help="Set the scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_witness_count', dest="block_tx_witness_count", default=0, type=int, help="Set the number of transaction witness itens")
    parser_createblock.add_argument('--invalid_tx_witness_count', dest="block_invalid_tx_witness_count", default=0, type=int, help="Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)")
    parser_createblock.add_argument('--tx_witness_size', dest="block_tx_witness_size", default=0, type=int, help="Set the item size from witness item of transactions as random invalid parameter")
    parser_createblock.add_argument('--tx_witness_item', dest="block_tx_witness_item", default=False, type=bool, help="Set the item from witness of transactions as random invalid parameter")

    # createtx arguments
    parser_createtx.add_argument('--tx_count', dest="tx_count", default=0, type=int, help="Set a number of transactions in block")
    parser_createtx.add_argument('--invalid_tx_count', dest="invalid_tx_count", default=0, type=int, help="Set a number of invalid transactions in block (if is not set, all transactions will be invalid)")
    parser_createtx.add_argument('--tx_version', dest="tx_version", default=False, type=bool, help="Set the transaction version as random invalid parameter")
    parser_createtx.add_argument('--tx_marker', dest="tx_marker", default=False, type=bool, help="Set the transaction market as random invalid parameter")
    parser_createtx.add_argument('--tx_flag', dest="tx_flag", default=False, type=bool, help="Set the transaction flag as random invalid parameter")
    parser_createtx.add_argument('--tx_locktime', dest="tx_locktime", default=False, type=bool, help="Set the transaction locktime as random invalid parameter")
    parser_createtx.add_argument('--tx_in_count', dest="tx_in_count", default=0, type=int, help="Set the number of transaction inputs")
    parser_createtx.add_argument('--invalid_tx_in_count', dest="invalid_tx_in_count", default=0, type=int, help="Set the number of invalid transaction inputs (if not set, all inputs will be invalid)")
    parser_createtx.add_argument('--tx_in_txid', dest="tx_in_txid", default=False, type=bool, help="Set the txid from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_vout', dest="tx_in_vout", default=False, type=bool, help="Set the vout from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_script_size', dest="tx_in_script_size", default=0, type=int, help="Set the size of scriptsig from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_script', dest="tx_in_script", default=False, type=bool, help="Set the scriptsig from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_in_sequence', dest="tx_in_sequence", default=False, type=bool, help="Set the sequence from inputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_out_count', dest="tx_out_count", default=0, type=int, help="Set the number of transaction outputs")
    parser_createtx.add_argument('--invalid_tx_out_count', dest="invalid_tx_out_count", default=0, type=int, help="Set the number of invalid transaction outputs (if not set, all outputs will be invalid)")
    parser_createtx.add_argument('--tx_out_amount', dest="tx_out_amount", default=0, type=int, help="Set the amount from outputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_out_script_size', dest="tx_out_script_size", default=0, type=int, help="Set the size of scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_out_script', dest="tx_out_script", default=False, type=bool, help="Set the scriptpubkey from outputs of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_witness_count', dest="tx_witness_count", default=0, type=int, help="Set the number of transaction witness itens")
    parser_createtx.add_argument('--invalid_tx_witness_count', dest="invalid_tx_witness_count", default=0, type=int, help="Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)")
    parser_createtx.add_argument('--tx_witness_size', dest="tx_witness_size", default=0, type=int, help="Set the item size from witness item of transactions as random invalid parameter")
    parser_createtx.add_argument('--tx_witness_item', dest="tx_witness_item", default=False, type=bool, help="Set the item from witness of transactions as random invalid parameter")

    args = parser.parse_args()

    if args.commands is None:
        parser.print_help()
    elif args.commands == "createblock":
        # Run createblock
        # createblock()
        print("createblock")
    elif args.commands == "createtx":
        # Run createtx
        # createtx()
        print("createtx")
    else:
        parser.print_help()
        sys.exit("No command provided.")

if __name__ == "__main__":
    main()