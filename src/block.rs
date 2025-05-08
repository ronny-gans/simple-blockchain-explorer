use chrono::prelude::*;
use sha2::{Sha256,Digest};
use serde::{Deserialize, Serialize};
use hex;

//initialize struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub data: u32,
    pub prev_hash: String,
    pub hash: String
}
impl Block {
    fn calculate_hash (&self) ->String {
        let block_for_serialization = Block {
            index: self.index,
            timestamp: self.timestamp,
            data:self.data,
            prev_hash:self.prev_hash.clone(),
            hash: String::new()
        };
        //serialize to JSON
        let serialized = serde_json::to_string(&block_for_serialization).expect("serialization failed");

        // compute sha 256
        let mut hasher = Sha256::new();
        hasher.update(serialized.as_bytes());
        let hash_bytes:[u8;32] = hasher.finalize().into();
        // convert to hex string
        hex::encode(hash_bytes)
    }
    pub fn new (index:u32,data:u32,prev_hash:String)->Self {
        let timestamp = Utc::now().timestamp_millis();
        let mut temp_block= Block {
            index,
            timestamp,
            data,
            prev_hash,
            hash:String::new()
        };
        temp_block.hash=temp_block.calculate_hash();
        temp_block
    }
}
