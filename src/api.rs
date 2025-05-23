use bitcoin::consensus::encode;
use bitcoin::block::Header;
use misfit_core::breakers::decoder_tools::{DecodedTransaction};
use misfit_core::transaction::random::transaction::TxParams;
use misfit_core::transaction::generator::GenerateTx;
use misfit_core::block::generate_blocks::GenerateBlock;
use misfit_core::breakers::{decoder_tools, };

pub struct Generator {}

impl Generator {
    pub fn block(tx_count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..tx_count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();
            let tx_id = tx.compute_txid().to_string();

            raw_tx.push(raw_transaction);
            txid.push(tx_id);
        }

        let block_header = GenerateBlock::new(txid.clone());

        [
            format!("Blockheader Info 🧊: {:#?} ", block_header),
            format!("Raw transactions used in it:{:#?}", raw_tx),
            format!("Used Txids: {:#?}", txid),
        ]
        .join("\n---\n")
    }

    // TODO: Implement params into transaction generator
    pub fn transaction(count: u32) -> String {
        let mut raw_tx: Vec<String> = vec![];
        let mut txid: Vec<String> = vec![];

        for _c in 0..count {
            let tx = GenerateTx::valid_random(TxParams::default());
            let raw_transaction = hex::encode(encode::serialize(&tx)).to_string();
            let tx_id = tx.compute_txid().to_string();

            raw_tx.push(raw_transaction);
            txid.push(tx_id);
        }

        [
            format!("Raw Transactions: {:#?}", raw_tx),
            format!("TXIDs: {:#?}", txid),
        ]
        .join("\n---\n")
    }

    pub fn decode_raw_transaction(raw_tx:String) -> Result<DecodedTransaction,Box<dyn std::error::Error>> {
        let decoder = decoder_tools::BitcoinTransactionDecoder::new();
        let decoded = decoder.decode_hex(&raw_tx);
        decoded    
    }

    pub fn decoder_block_header(block_header:String) -> Result<Header, Box<dyn std::error::Error>> {
        decoder_tools::BlockUtils::decode_header_from_hex(&block_header)
    }
/* 
    pub fn break_transaction(flags:Vec<String>)-> String{
        
    }
    pub fn break_block(flags:Vec<String>)-> String{

    }
*/
}
