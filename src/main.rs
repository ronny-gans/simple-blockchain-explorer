use std::io;
mod blockchain;
mod block;
use block::Transaction;
use crate::blockchain::Blockchain;


fn main() {
    let mut chain=Blockchain::new();
    loop {
        println!("menu:");
        let menus = ["add new transaction","show all transaction", "save transaction to file", "load transaction from file","exit"];
        for menu in menus {
            println!("{}",menu)
        }
        let mut choose_menu = String::new();
        io::stdin()
            .read_line(&mut choose_menu)
            .expect("failed to get value");
        let choose_menu = choose_menu.trim();
        match choose_menu {
            "add new transaction" => {
                loop {
                    println!("enter the sender name: ");
                    let mut sender_name = String::new();
                    io::stdin()
                        .read_line(&mut sender_name)
                        .expect("failed to get sender name");
                    let sender_name = sender_name.trim();
                    println!("enter the receiver name: ");
                    let mut receiver_name = String::new();
                    io::stdin() 
                        .read_line(&mut receiver_name)
                        .expect("failed to get receiver name");
                    let receiver_name = receiver_name.trim();
                    println!("enter the transaction value!");
                    let mut transaction_value = String::new();
                    io::stdin()
                        .read_line(&mut transaction_value)
                        .expect("failed to get transaction value");
                    let transaction_value:u32 = match transaction_value.trim().parse() {
                        Ok(num)=> num,
                        Err(_) =>continue
                    };
                    chain.add_block(vec![Transaction {
                        sender:sender_name.to_string(),
                        receiver:receiver_name.to_string(),
                        value:transaction_value
                    }]);
                    println!("want to add more transaction? [y/n]");
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
            }
            "show all transaction"=> chain.read_blocks(),
            "save transaction to file" => {
                println!("save your name as:");
                let mut name_file = String::new();
                io::stdin()
                    .read_line(&mut name_file)
                    .expect("failed to get filename");
                let name_file = name_file.trim();
                match chain.save_to_file(name_file) {
                    Ok(_) => println!("file saved successfully"),
                    Err(e) => println!("Failed to save blockchain: {}",e),
                }
            }
            "load transaction from file" => {
                println!("input your file path");
                let mut file_path = String::new();
                io::stdin()
                    .read_line(&mut file_path)
                    .expect("failed to get file");
                let file_path =file_path.trim();
                match Blockchain::read_from_file(file_path) {
                    Ok(loaded_chain) => {
                        chain=loaded_chain;
                        println!("blockchain loaded successfully")
                    }
                    Err(e) => {
                        println!("Failed to load blockchain: {}",e)
                    }
                }
            }
            "exit" => break,
            _ => println!("choose the right menu"),

        }
    
    }
}
