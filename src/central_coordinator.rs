use tokio::sync::mpsc;
use crate::worker::worker_node;
use crate::task_model::{Subtask, TaskResult};
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
        let task_rx_clone = Arc::clone(&shared_task_rx); // Use the shared receiver
        let result_tx_clone = result_tx.clone();

        tokio::spawn(async move {
            worker_node(worker_id, task_rx_clone, result_tx_clone).await;
        });
    }

    // Simulate task creation
    for i in 0..10 {
        let task = Subtask {
            id: i,
            description: format!("Task {}", i),
        };
        task_tx.send(task).await.unwrap();
    }

    drop(task_tx); // Close task sender after all tasks are sent

    // Collect results
    while let Some(result) = result_rx.recv().await {
        println!("Coordinator received result: {:?}", result);
    }
}
