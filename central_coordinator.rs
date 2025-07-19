use tokio::sync::mpsc;
use crate::worker::worker_node;
use crate::task_model::{Subtask, TaskResult, TaskStep};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn task_distributor() {
    let (task_tx, task_rx) = mpsc::channel(100);
    let (result_tx, mut result_rx) = mpsc::channel(100);

    // Share a single `Receiver` for all workers
    let shared_task_rx = Arc::new(Mutex::new(task_rx));

    // Launch worker nodes
    for i in 0..3 {
        let worker_id = i;
        let task_rx_clone = Arc::clone(&shared_task_rx);
        let result_tx_clone = result_tx.clone();

        tokio::spawn(async move {
            worker_node(worker_id, task_rx_clone, result_tx_clone).await;
        });
    }

    // Define a list of websites to visit
    let websites = vec![
        "https://example.com",
        "https://rust-lang.org",
        "https://tokio.rs",
        "https://crates.io",
        "https://docs.rs",
    ];

    // Simulate task creation with website URLs
    for (i, url) in websites.iter().enumerate() {
        let open_website_task = Subtask {
            id: i * 2,
            description: format!("Open website: {}", url),
            step: TaskStep::OpenWebsite(url.to_string()),
        };

        let categorize_task = Subtask {
            id: i * 2 + 1,
            description: format!("Categorize elements on: {}", url),
            step: TaskStep::CategorizeElements,
        };

        // Send the `OpenWebsite` and `CategorizeElements` tasks
        task_tx.send(open_website_task).await.unwrap();
        task_tx.send(categorize_task).await.unwrap();
    }

    drop(task_tx); // Close task sender after all tasks are sent

    // Collect results
    while let Some(result) = result_rx.recv().await {
        println!("Coordinator received result: {:?}", result);
    }
}
