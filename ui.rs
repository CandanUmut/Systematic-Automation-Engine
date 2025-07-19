use crate::{automation_engine, task_library, rewards};
use std::io;

pub async fn start_ui() {
    loop {
        println!("Systematic Automation Framework:");
        println!("1. Create Automation Task");
        println!("2. Run Automation Task");
        println!("3. View Rewards");
        println!("4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "1" => automation_engine::create_task().await,
            "2" => automation_engine::run_task().await,
            "3" => rewards::view_rewards(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
