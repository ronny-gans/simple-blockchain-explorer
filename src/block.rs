// use std::clone;

use chrono::prelude::*;
use sha2::{Sha256,Digest};
use serde::{Deserialize, Serialize};
use hex;

//initialize data (transaction)
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct  Transaction {
    pub sender: String,
    pub receiver: String,
    pub value: u32
}

//initialize struct
#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub data: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String
}
impl Block {
    fn calculate_hash (&self) ->String {
        let block_for_serialization = Block {
            index: self.index,
            timestamp: self.timestamp,
            data:self.data.clone(),
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
    pub fn new (index:u32,data:Vec<Transaction>,prev_hash:String)->Self {
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
    // new method to format timestamp
    pub fn format_timestamp(&self) -> String {
        // Convert i64 timestamp (milliseconds) to DateTime<Utc>
        let datetime = DateTime::<Utc>::from_timestamp_millis(self.timestamp)
            .expect("invalid timestamp");
        // Format as ISO 8601 (e.g., "2025-05-09T12:34:56.789Z")
        datetime.to_rfc3339()
    }
}
