class BitcoinBlockGenerator:
    def __init__(self, version=1, previous_block_hash="0"*64, merkle_root="0"*64, timestamp=None, difficulty_target=0x1d00ffff, nonce=0):
        """
        Initialize a Bitcoin block generator with default or preconfigured values.

        :param version: Block version (default is 1)
        :param previous_block_hash: Hash of the previous block (default is a string of 64 zeros)
        :param merkle_root: Merkle root of the transactions (default is a string of 64 zeros)
        :param timestamp: Timestamp of the block (default is current time if None)
        :param difficulty_target: Difficulty target for mining (default is 0x1d00ffff)
        :param nonce: Nonce value for mining (default is 0)
        """
        self.version = version
        self.previous_block_hash = previous_block_hash
        self.merkle_root = merkle_root
        self.timestamp = timestamp if timestamp is not None else self.get_current_timestamp()
        self.difficulty_target = difficulty_target
        self.nonce = nonce

    def get_current_timestamp(self):
        """
        Get the current timestamp in seconds since the epoch.
        """
        import time
        return int(time.time())

    def set_version(self, version):
        """
        Set the block version.
        """
        self.version = version

    def set_previous_block_hash(self, previous_block_hash):
        """
        Set the previous block hash.
        """
        self.previous_block_hash = previous_block_hash

    def set_merkle_root(self, merkle_root):
        """
        Set the Merkle root.
        """
        self.merkle_root = merkle_root

    def set_timestamp(self, timestamp):
        """
        Set the block timestamp.
        """
        self.timestamp = timestamp

    def set_difficulty_target(self, difficulty_target):
        """
        Set the difficulty target.
        """
        self.difficulty_target = difficulty_target

    def set_nonce(self, nonce):
        """
        Set the nonce value.
        """
        self.nonce = nonce

    def generate_block_header(self):
        """
        Generate the block header using the current values.
        """
        block_header = {
            "version": self.version,
            "previous_block_hash": self.previous_block_hash,
            "merkle_root": self.merkle_root,
            "timestamp": self.timestamp,
            "difficulty_target": self.difficulty_target,
            "nonce": self.nonce
        }
        return block_header

    def user_input_block(self):
        """
        Allow the user to input their own values for the block.
        """
        self.version = int(input("Enter block version: ") or self.version)
        self.previous_block_hash = input("Enter previous block hash: ") or self.previous_block_hash
        self.merkle_root = input("Enter Merkle root: ") or self.merkle_root
        self.timestamp = int(input("Enter timestamp: ") or self.timestamp)
        self.difficulty_target = int(input("Enter difficulty target: ") or self.difficulty_target)
        self.nonce = int(input("Enter nonce: ") or self.nonce)

    def __str__(self):
        """
        Return a string representation of the block header.
        """
        block_header = self.generate_block