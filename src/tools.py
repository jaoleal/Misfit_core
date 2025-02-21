from subprocess import run
import string
import random

def randomize(str):
    new_str = ""
    for c in str:
        new_str += random.choice(string.hexdigits)
    return new_str


def bcli(cmd: str):
    res = run(
            ["bitcoin-cli", "-regtest"] + cmd.split(" "), capture_output=True, encoding="utf-8")
    if res.returncode == 0:
        return res.stdout.strip()
    else:
        raise Exception(res.stderr.strip())
    
#print(randomize("04000020ab8e65b104e01f2fa16395a8a6f315bea0049cdc8dac0a00000000000000000034c7a1e1ac24b459ad245d66e66995d6a8c9b520bc5207282d51ba3bce7f5717e04f776108040e175375b835"))