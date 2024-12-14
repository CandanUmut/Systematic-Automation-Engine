use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref REWARDS: Mutex<u64> = Mutex::new(0);
}

pub fn add_reward(amount: u64) {
    let mut rewards = REWARDS.lock().unwrap();
    *rewards += amount;
}

pub fn view_rewards() {
    let rewards = REWARDS.lock().unwrap();
    println!("Total TimeCoin Rewards: {}", rewards);
}
