use std::io;
use std::io::Write;
use std::process;

mod blockchain;
use blockchain::Chain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty
        .trim()
        .parse::<u32>()
        .expect("we need as integer");
    println!("generating genesis block!");
    let mut chain = Chain::new(miner_address.trim().to_string(), diff);
    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("exiting..");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter Sender address:");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("Enter receiver address");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Enter amount");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated succesfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter a new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Successfully"),
                    false => println!("Failed to update difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Update reward"),
                    false => println!("Failed to update reward"),
                }
            }
            _ => println!("Invalid Option Retry"),
        }
    }
}
