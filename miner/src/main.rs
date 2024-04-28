use miner::structs::Transaction;
use miner::utils::print_hex_string;
use miner::{calculate_merkle_root, create_block_header, create_coinbase_trx, mine_block, print_soln};
use std::error::Error;
use std::fs::read_dir;
fn main() -> Result<(), Box<dyn Error>> {
    let directory_path = "../mempool";
    let mut txids: Vec<Vec<u8>> = Vec::new();
    // let mut transactions: Vec<u8> = Vec::new();
    let crx = create_coinbase_trx();
    txids.push(crx.1);
    // transactions.extend(crx.0);
    for entry in read_dir(directory_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "json" {
            let transaction = Transaction::parse_from_file(path.to_str().unwrap())?;

            // println!("--- Parsed file: {}", path.display());
            let isegwit = transaction.valid_trans();
            if isegwit.0 {
                // let serialized_data = transaction.get_data(isegwit.1);
                // print!("Hashed Data ");
                // print_hex_string(&serialized_data);
                // transactions.extend(serialized_data);
                // print!("Transaction Id ");
                let txis = transaction.get_txid();
                txids.push(txis);
                // print_hex_string(&txis);
            }
        }
    }
    let merkle_root = calculate_merkle_root(&txids);
    let block_header = create_block_header(merkle_root);
    let (block,_nonce) = mine_block(&block_header);
    print_soln(&block, &txids);
    Ok(())
}
