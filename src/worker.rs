use tokio::sync::mpsc;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::task_model::{Subtask, TaskResult};

pub async fn worker_node(
    worker_id: usize,
    task_rx: Arc<Mutex<mpsc::Receiver<Subtask>>>,
    result_tx: mpsc::Sender<TaskResult>,
) {
    println!("Worker {} started", worker_id);

    while let Some(task) = task_rx.lock().await.recv().await {
        println!("Worker {} processing task: {:?}", worker_id, task);

        // Simulate task execution
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Send result back to the coordinator
        let result = TaskResult {
            id: task.id,
            status: format!("Completed by Worker {}", worker_id),
        };
        result_tx.send(result).await.unwrap();
    }

    println!("Worker {} exiting", worker_id);
}
