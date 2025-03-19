import pytest
from importlib import import_module

from utils import merkleroot, reverse_bytes

class TestExample:
    def merkleroot_test(self):
        merkleroot(['dba724cbd65bb986b2c2111061e0d86af9d5ad0bdd78fbefdee33a1eefec4ec3',
      '21a2ea05ed5b7235c9b6bc82e457f184c9e21ca078f50e93c526258a91449b4c']) == "7deff0606adb42e5ee4bff7a0ebc56392f65fb5e34401dce596c18395da9d8e4"

    def reverse_test(self):
        reverse_bytes("7deff0606adb42e5ee4bff7a0ebc56392f65fb5e34401dce596c18395da9d8e4") == "e4d8a95d39186c59ce1d40345efb652f3956bc0e7aff4beee542db6a60f0ef7d"

