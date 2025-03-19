from importlib import import_module
import sys
import os
from src.test_case import runner


COMMANDS_DIR = "commands"

def main():
    os.makedirs(COMMANDS_DIR, exist_ok=True)
    sys.exit(runner())

if __name__ == "__main__":
    main()
