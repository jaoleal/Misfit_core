# Misfit-Core

A tool for create specified invalid parameters for tests in bicoin.

## Documentation

## run a regtest
For run a regtest, with the bitcoin binaries installed
```b
bitcoind -regetst
```
## Installation
Clone the repo
```b
git clone https://github.com/j-moreno-c-r/Misfit_core
```
Enter the directory
```b
cd Misfit_core
```
Compile the project with Cargo
```b
cargo build 
```
run the binary 
```b
./target/debug/Misfit_core 
```
Or run with the cargo 
```b
cargo run -- --"flag you want to use" "another flags or arguments"
```
### Basic usage

<u>Usage:</u> `misfit-core [OPTIONS] <COMMAND>`

<u>Commands:</u>
- <b>createblock</b> Create a block with invalid specified parameters
- <b>createtx</b> Create transaction with invalid specified parameters

<u>Options:</u>
- <b>--help</b> Show this help message

### Create a misfit block

Create a block with invalid specified parameters

<u>Usage:</u> `misfit-core createblock [OPTIONS]`

<u>Options:</u>
- <b>--help</b> Show this help message

- <u><b>Block:</b></u>
  - <b>--version</b> Set the block version as random invalid parameter

  - <b>--prevblock</b> Set the previous block hash as random invalid parameter

  - <b>--merkleroot</b> Set the merkle root as random invalid parameter

  - <b>--timestamp</b> Set the timestamp as random invalid parameter

  - <b>--bits</b> Set the bits as random invalid parameter

  - <b>--nonce</b> Set the nonce as random invalid parameter

  
- <u><b>Transactions:</b></u>
  - <b>--tx_count</b> Set a number of transactions in block
  
  - <b>--invalid_tx_count</b> Set a number of invalid transactions in block (if is not set, all transactions will be invalid)
  
  - <b>--tx_version</b> Set the transaction version as random invalid parameter
  
  - <b>--tx_marker</b> Set the transaction market as random invalid parameter
  
  - <b>--tx_flag</b> Set the transaction flag as random invalid parameter
  
  - <b>--tx_locktime</b> Set the transaction locktime as random invalid parameter
  
  - <u><b>Inputs:</b></u>
    - <b>--tx_in_count</b> Set the number of transaction inputs
    
    - <b>--invalid_tx_in_count</b> Set the number of invalid transaction inputs (if not set, all inputs will be invalid)
    
    - <b>--tx_in_txid</b> Set the txid from inputs of transactions as random invalid parameter
    
    - <b>--tx_in_vout</b> Set the vout from inputs of transactions as random invalid parameter
    
    - <b>--tx_in_script_size</b> Set the size of scriptsig from inputs of transactions as random invalid parameter
    
    - <b>--tx_in_script</b> Set the scriptsig from inputs of transactions as random invalid parameter
    
    - <b>--tx_in_sequence</b> Set the sequence from inputs of transactions as random invalid parameter
  
  - <u><b>Outputs:</b></u>
    - <b>--tx_out_count</b> Set the number of transaction outputs
    
    - <b>--invalid_tx_out_count</b> Set the number of invalid transaction outputs (if not set, all outputs will be invalid)
    
    - <b>--tx_out_amount</b> Set the amount from outputs of transactions as random invalid parameter
    
    - <b>--tx_out_script_size</b> Set the size of scriptpubkey from outputs of transactions as random invalid parameter
    
    - <b>--tx_out_script</b> Set the scriptpubkey from outputs of transactions as random invalid parameter
  
  - <u><b>Witness:</b></u>
    - <b>--tx_witness_count</b> Set the number of transaction witness itens
    
    - <b>--invalid_tx_witness_count</b> Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)
    
    - <b>--tx_witness_size</b> Set the item size from witness item of transactions as random invalid parameter
    
    - <b>--tx_witness_item</b> Set the item from witness of transactions as random invalid parameter

### Create a misfit transaction

Create transaction with invalid specified parameters

<u>Usage:</u> `misfit-core createtx [OPTIONS]`

<u>Options:</u>
- <b>--help</b> Show this help message

- <b>--tx_count</b> Set a number of transactions in block
  
- <b>--invalid_tx_count</b> Set a number of invalid transactions in block (if is not set, all transactions will be invalid)

- <b>--tx_version</b> Set the transaction version as random invalid parameter

- <b>--tx_marker</b> Set the transaction market as random invalid parameter

- <b>--tx_flag</b> Set the transaction flag as random invalid parameter

- <b>--tx_locktime</b> Set the transaction locktime as random invalid parameter

- <u><b>Inputs:</b></u>
  - <b>--tx_in_count</b> Set the number of transaction inputs
  
  - <b>--invalid_tx_in_count</b> Set the number of invalid transaction inputs (if not set, all inputs will be invalid)
  
  - <b>--tx_in_txid</b> Set the txid from inputs of transactions as random invalid parameter
  
  - <b>--tx_in_vout</b> Set the vout from inputs of transactions as random invalid parameter
  
  - <b>--tx_in_script_size</b> Set the size of scriptsig from inputs of transactions as random invalid parameter
  
  - <b>--tx_in_script</b> Set the scriptsig from inputs of transactions as random invalid parameter
  
  - <b>--tx_in_sequence</b> Set the sequence from inputs of transactions as random invalid parameter

- <u><b>Outputs:</b></u>
  - <b>--tx_out_count</b> Set the number of transaction outputs
  
  - <b>--invalid_tx_out_count</b> Set the number of invalid transaction outputs (if not set, all outputs will be invalid)
  
  - <b>--tx_out_amount</b> Set the amount from outputs of transactions as random invalid parameter
  
  - <b>--tx_out_script_size</b> Set the size of scriptpubkey from outputs of transactions as random invalid parameter
  
  - <b>--tx_out_script</b> Set the scriptpubkey from outputs of transactions as random invalid parameter

- <u><b>Witness:</b></u>
  - <b>--tx_witness_count</b> Set the number of transaction witness itens
  
  - <b>--invalid_tx_witness_count</b> Set the number of invalid transaction witness itens (if not set, all witness itens will be invalid)
  
  - <b>--tx_witness_size</b> Set the item size from witness item of transactions as random invalid parameter
  
  - <b>--tx_witness_item</b> Set the item from witness of transactions as random invalid parameter