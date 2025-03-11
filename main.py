from argparse import ArgumentDefaultsHelpFormatter, ArgumentParser, Namespace
from importlib import import_module
import sys
import os
from typing import List, Optional

from src.misfit_core import MisfitCore


COMMANDS_DIR = "commands"

def main():
    os.makedirs(COMMANDS_DIR, exist_ok=True)
    sys.exit(MisfitCore())

if __name__ == "__main__":
    main()
