from subprocess import run
import string
import random


def randomize(str) -> str:
    return random.randbytes(len(bytes.fromhex(str))).hex()


def bcli(cmd: str):
    res = run(
        ["bitcoin-cli", "-regtest"] + cmd.split(" "), capture_output=True, encoding="utf-8")
    if res.returncode == 0:
        return res.stdout.strip()
    else:
        raise Exception(res.stderr.strip())
