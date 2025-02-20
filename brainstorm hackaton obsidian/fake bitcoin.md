1 step -> do a creator of valid random blocks with valid random transactions, its nice has inputs to especify the mx number of txs and type of that.
2 -> do a inputet for the user(dev tester) especify how many and what campus he what to break with invalid data
3 -> do something to process and generate the final block with especific campus braked.
4 -> its interesing implemet something to import a real block to break by block header in this case we wil need to implement a block splitter.

things that i consider important in the breake options: 
* merkle root
* txid of one of the transasctions
* general campus of txs 
	* input
	* outputs
	* signatures
* block hash
* block header
* version block
* version number
* timestamp
* dificult target
* nonce
we can use this https://ratatui.rs/ for a tui interface.

we can use this: https://medium.com/@Collinszurum/bitcoin-block-construction-using-rust-a-step-by-step-guide-2c961d5e1af2
for the base to construct the block
and some slides with escalidraw and google slides for presentation.