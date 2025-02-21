from bitcoinlib.blocks import Block
from bitcoinlib.transactions import Transaction
from bitcoinlib.keys import Key
import random
import time

def generate_random_transaction():
    # Generate random keys for sender and receiver
    sender_key = Key()
    receiver_key = Key()

    # Create a random transaction
    transaction = Transaction(
        inputs=[{'output': '00' * 32 + ':0', 'value': 100000}],  # Dummy input
        outputs=[{'value': 50000, 'address': receiver_key.address()}],
        network='bitcoin'
    )
    transaction.sign(sender_key)
    return transaction

def generate_random_block(previous_block_hash):
    # Generate a random block
    block = Block(
        version=1,
        prev_block=previous_block_hash,
        merkle_root='00' * 32,  # Placeholder, will be updated
        timestamp=int(time.time()),
        bits=0x1d00ffff,  # Difficulty target
        nonce=0
    )

    # Add random transactions to the block
    num_transactions = random.randint(1, 10)
    for _ in range(num_transactions):
        transaction = generate_random_transaction()
        block.add_transaction(transaction)

    # Update the merkle root
    block.merkle_root = block.calculate_merkle_root()

    # Mine the block (find a valid nonce)
    block.mine()

    return block

# Example usage
if __name__ == "__main__":
    previous_block_hash = '0000000000000000000000000000000000000000000000000000000000000000'  # Genesis block hash
    random_block = generate_random_block(previous_block_hash)
    print("Generated Block:")
    print(random_block.to_json())