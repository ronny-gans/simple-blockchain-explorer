
use crate::block::{Block, Transaction};
use serde::{Serialize,Deserialize};
use serde_json;
use std::io::{self,Write,Read};
use std::fs::File;

// initialize blockchain
#[derive(Serialize,Deserialize)]
pub struct  Blockchain {
    blocks:Vec<Block>
}
impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }
    pub fn add_block(&mut self, data:Vec<Transaction>) {
        let index=self.blocks.len() as u32;
        let prev_hash= if let Some(last)=self.blocks.last() {
            last.hash.clone()
        } else {
            String::from("0") // genesis block
        };
        let new_block=Block::new(index,data,prev_hash);
        self.blocks.push(new_block)
        
    }
    pub fn save_to_file (&self,filename:&str) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self.blocks).unwrap();
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    pub fn read_from_file (filename:&str) -> io::Result<Blockchain> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let blocks:Vec<Block>=serde_json::from_str(&contents).unwrap(); //if there is no file load
        Ok(Blockchain{blocks})
    }
    pub fn read_blocks(&self) {
        for block in &self.blocks {
            println!(
                "block: #{}, timestamp = {}, hash = {}, previous hash = {}",
                block.index,
                block.format_timestamp(),
                block.hash,
                block.prev_hash
        )
        }
    }
}
