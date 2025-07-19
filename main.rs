mod automation_engine;
mod web_interaction;
mod task_library;
mod ui;
mod executor;
mod central_coordinator;
mod worker;
mod task_model;
mod rewards;


use central_coordinator::task_distributor;

#[tokio::main]
async fn main() {
    println!("🚀 Welcome to the Systematic Automation Framework!");
    
    // Start the Central Coordinator
    task_distributor().await;
}