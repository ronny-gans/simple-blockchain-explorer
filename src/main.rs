use std::io;
mod blockchain;
mod block;
use crate::blockchain::Blockchain;


fn main() {
    let mut chain=Blockchain::new();
    loop {
        println!("enter a transaction value");
        let mut transaction = String::new();
        io::stdin()
            .read_line(&mut transaction)
            .expect("failed to get transaction");
        let transaction:u32 = match transaction.trim().parse() {
            Ok(num) => num,
            Err(_) =>  continue
        };
        chain.add_block(transaction);
        println!("do you want add more transaction? [y/n]");
        let mut confirmation = String::new();
        io::stdin()
            .read_line(&mut confirmation)
            .expect("enter y or n");
        let confirmation = confirmation.trim();
        match confirmation {
            "y" => continue,
            "n" => break,
            _ => println!("enter [y/n]!"),
        }
    }
    for block in chain.iter() {
        println!("Block #{}, hash = {}, prev_hash={}, time: {}", block.index, block.hash, block.prev_hash, block.timestamp);
    }
    
}
