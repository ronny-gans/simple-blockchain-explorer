
use crate::block::Block;
// initialize blockchain
pub struct  Blockchain {
    blocks:Vec<Block>
}
impl Blockchain {
    pub fn iter(&self) -> std::slice::Iter<'_, Block> {
        self.blocks.iter()
    }
    pub fn new() -> Blockchain {
        Blockchain { blocks: Vec::new() }
    }
    pub fn add_block(&mut self, data:u32) {
        let index=self.blocks.len() as u32;
        let prev_hash= if let Some(last)=self.blocks.last() {
            last.hash.clone()
        } else {
            String::from("0") // genesis block
        };
        let new_block=Block::new(index,data,prev_hash);
        self.blocks.push(new_block)
        
    }
}
