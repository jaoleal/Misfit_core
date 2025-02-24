from argparse import ArgumentDefaultsHelpFormatter, ArgumentParser, Namespace
from importlib import import_module
import sys
import os
from typing import List, Optional

COMMANDS_DIR = "commands"

class MisfitCore:
    def __init__(self, args: Optional[List[str]] = None) -> str:
        parsed = self.parse_args(args)

        if not parsed.command:
            self.parser.print_help()
            sys.exit("No command provided.")

        module_path = f"src.{COMMANDS_DIR}.{parsed.command}"
        module = import_module(module_path)
        
        if hasattr(module, 'execute'):
            return module.execute(parsed)

    def get_commands(self) -> list[str]:
        commands = []

        if not os.path.exists(COMMANDS_DIR):
            return commands
        
        # Get all python files inside COMMANDS_DIR
        for filename in os.listdir(f"src/{COMMANDS_DIR}"):
            if filename.endswith(".py") and not filename.startswith("__"):
                commands.append(filename[:-3])

        return commands

    def parse_args(self, args: Optional[list[str]] = None) -> Namespace:
        commands = self.get_commands()

        self.parser = ArgumentParser(
            description="A tool for create specified invalid parameters for tests in bicoin.",
            formatter_class=ArgumentDefaultsHelpFormatter
        )
        
        self.parser.add_argument('--verbose', action='store_true', help='Enable verbose output')
        # TODO: parser.add_argument('--config', default='config.ini', help='Configuration file')
        
        subparsers = self.parser.add_subparsers(title="Commands", dest="command", help="Available commands")

        for pyfile in commands:
            module_path = f"src.{COMMANDS_DIR}.{pyfile}"
            module = import_module(module_path)
            module.register_parser(subparsers)
        
        return self.parser.parse_args(args)

# def main():
#     parser = ArgumentParser(
#         description="A tool for create specified invalid parameters for tests in bicoin.")
#     subparsers = parser.add_subparsers(title="Commands", dest="commands")

#     # Commands
#     parser_createblock = subparsers.add_parser(
#         "createblock", help="Create a block with invalid specified parameters")
#     parser_createtx = subparsers.add_parser(
#         "createtx", help="Create transaction with invalid specified parameters")

#     # createblock arguments
#     parser_createblock.add_argument('--version',
#                                     action="store_true", help="Set the block version as random invalid parameter")
#     parser_createblock.add_argument('--prevblock',
#                                     action="store_true", help="Set the previous block hash as random invalid parameter")
#     parser_createblock.add_argument('--merkleroot',
#                                     action="store_true", help="Set the merkle root as random invalid parameter")
#     parser_createblock.add_argument('--timestamp',
#                                     action="store_true", help="Set the timestamp as random invalid parameter")
#     parser_createblock.add_argument('--bits',
#                                     action="store_true", help="Set the bits as random invalid parameter")
#     parser_createblock.add_argument('--nonce',
#                                     action="store_true", help="Set the nonce as random invalid parameter")
#     parser_createblock.add_argument('--tx_count', dest="block_tx_count",
#                                     default=0, type=int, help="Set a number of transactions in block")
#     parser_createblock.add_argument('--invalid_tx_count', default=0, type=int,
#                                     help="Set a number of invalid transactions in block (if is not set, all transactions will be invalid)")
#     parser_createblock.add_argument('--tx_version',
#                                     action="store_true", help="Set the transaction version as random invalid parameter")
#     parser_createblock.add_argument('--tx_marker',
#                                     action="store_true", help="Set the transaction market as random invalid parameter")
#     parser_createblock.add_argument('--tx_flag',
#                                     action="store_true", help="Set the transaction flag as random invalid parameter")
#     parser_createblock.add_argument('--tx_locktime',
#                                     action="store_true", help="Set the transaction locktime as random invalid parameter")
#     parser_createblock.add_argument('--tx_in_count', dest="block_tx_in_count",
#                                     default=0, type=int, help="Set the number of transaction inputs")
#     parser_createblock.add_argument('--invalid_tx_in_count', default=0,
#                                     type=int, help="Set the number of invalid transaction inputs (if not set, all inputs will be invalid)")
#     parser_createblock.add_argument('--tx_in_txid',
#                                     action="store_true", help="Set the txid from inputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_in_vout',
#                                     action="store_true", help="Set the vout from inputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_in_script_size', default=0,
#                                     type=int, help="Set the size of scriptsig from inputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_in_script',
#                                     action="store_true", help="Set the scriptsig from inputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_in_sequence',
#                                     action="store_true", help="Set the sequence from inputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_out_count', dest="block_tx_out_count",
#                                     default=0, type=int, help="Set the number of transaction outputs")
#     parser_createblock.add_argument('--invalid_tx_out_count', default=0,
#                                     type=int, help="Set the number of invalid transaction outputs (if not set, all outputs will be invalid)")
#     parser_createblock.add_argument('--tx_out_amount', default=0,
#                                     type=int, help="Set the amount from outputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_out_script_size', default=0,
#                                     type=int, help="Set the size of scriptpubkey from outputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_out_script',
#                                     action="store_true", help="Set the scriptpubkey from outputs of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_witness_count', dest="block_tx_witness_count",
#                                     default=0, type=int, help="Set the number of transaction witness itens")
#     parser_createblock.add_argument('--invalid_tx_witness_count', default=0, type=int,
#                                     help="Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)")
#     parser_createblock.add_argument('--tx_witness_size', default=0,
#                                     type=int, help="Set the item size from witness item of transactions as random invalid parameter")
#     parser_createblock.add_argument('--tx_witness_item',
#                                     action="store_true", help="Set the item from witness of transactions as random invalid parameter")

def main():
    os.makedirs(COMMANDS_DIR, exist_ok=True)
    sys.exit(MisfitCore())

if __name__ == "__main__":
    main()
