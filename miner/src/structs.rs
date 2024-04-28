use serde::Deserialize;
use std::fs::File;
use serde_json::from_reader;
use crate::{calculate_txid, validate_transaction,serialize_transation};
use crate::utils::get_current_unix_timestamp_u32;
// Defining the structs
#[derive(Debug, Deserialize)]
pub struct Prevout {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: String,
    pub value: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Witness(pub Vec<String>);

#[derive(Debug, Deserialize)]
pub struct Vin {
    pub txid: String,
    pub vout: u32,
    pub prevout: Prevout,
    pub scriptsig: String,
    pub scriptsig_asm: String,
    pub witness: Option<Witness>,
    pub is_coinbase: bool,
    pub sequence: u32,
}

#[derive(Debug, Deserialize)]
pub struct Vout {
    pub scriptpubkey: String,
    pub scriptpubkey_asm: String,
    pub scriptpubkey_type: String,
    pub scriptpubkey_address: Option<String>,
    pub value: u64,
}
#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub version: u32,
    pub locktime: u32,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
}


impl Transaction {
    pub fn parse_from_file(file_path: &str) -> Result<Transaction, std::io::Error> {
        let file = File::open(file_path)?;
        let transaction: Transaction = from_reader(file)?;
        Ok(transaction)
    }

/// validates a transaction
///
/// # Arguments
///
/// * `self`: a `Transaction` object
///
/// # Returns
///
/// 'bool' the transaction is valid or not
    pub fn valid_trans(&self) -> (bool,bool) {

        // Preliminary test to find the total input > total output 
        let mut input = 0;
        let mut output = 0;
        for vin in self.vin.iter() {
            input += vin.prevout.value;
        }
        for vout in self.vout.iter() {
            output += vout.value;
        }
        // println!("Input {}, Output {}, Gas {}", input, output, input - output);
        if input <= output {
            return (false,false)
        }
        if self.locktime != 0 
        {
            if self.locktime < 499999999 
            {
                return  (false,false);
            }
            else {
                let unixtime = get_current_unix_timestamp_u32();
                if self.locktime > unixtime 
                {
                    return (false,false);
                }
            }
        }
        match validate_transaction(self) {
            Ok(flag) => return  (true,flag),
            Err(_err) => {
                // println!("Encountered error {}",err); 
                return (false,false);
            }
        }
    }
    pub fn get_data(&self,issegwit : bool) -> Vec<u8> 
    {
        return serialize_transation(self, issegwit);
    }
    pub fn get_txid(&self) -> Vec<u8> {
        return calculate_txid(self);
    }
}
