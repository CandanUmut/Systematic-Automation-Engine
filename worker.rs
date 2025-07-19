use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use crate::task_model::{Subtask, TaskResult, TaskStep};
use fantoccini::{Client, Locator};
use crate::web_interaction::universal_locator;
use crate::web_interaction::open_website;

pub async fn worker_node(
    worker_id: usize,
    task_rx: Arc<Mutex<mpsc::Receiver<Subtask>>>,
    result_tx: mpsc::Sender<TaskResult>,
) {
    println!("Worker {} started", worker_id);

    let mut client = match Client::new("http://localhost:4444").await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Worker {}: Failed to start browser client: {:?}", worker_id, e);
            return;
        }
    };

    while let Some(task) = task_rx.lock().await.recv().await {
        println!("Worker {} processing task: {:?}", worker_id, task);

        match task.step {
            TaskStep::OpenWebsite(url) => {
                // Navigate to a website
                match open_website(&mut client, &url).await {
                    Ok(_) => {
                        println!("Worker {}: Successfully opened website: {}", worker_id, url);
                        result_tx.send(TaskResult {
                            id: task.id,
                            status: format!("Opened website by Worker {}", worker_id),
                            details: Some(url.clone()),
                        }).await.unwrap();
                    }
                    Err(e) => {
                        eprintln!("Worker {}: Failed to open website: {:?}", worker_id, e);
                        result_tx.send(TaskResult {
                            id: task.id,
                            status: format!("Failed to open website by Worker {}: {:?}", worker_id, e),
                            details: None,
                        }).await.unwrap();
                    }
                }
            }
            
            TaskStep::CategorizeElements => {
                // Perform element categorization
                match universal_locator(&mut client).await {
                    Ok(categorized_elements) => {
                        println!(
                            "Worker {} categorized elements: {:?}",
                            worker_id, categorized_elements
                        );
                        result_tx.send(TaskResult {
                            id: task.id,
                            status: format!("Categorized elements by Worker {}", worker_id),
                            details: Some(format!("{:?}", categorized_elements)),
                        }).await.unwrap();
                    }
                    Err(e) => {
                        eprintln!("Worker {}: Failed to categorize elements: {:?}", worker_id, e);
                        result_tx.send(TaskResult {
                            id: task.id,
                            status: format!(
                                "Failed to categorize elements by Worker {}: {:?}",
                                worker_id, e
                            ),
                            details: None,
                        }).await.unwrap();
                    }
                }
            }
            TaskStep::ClickElement(selector) => {
                // Perform a click action
                match client.find(Locator::Css(&selector)).await {
                    Ok(mut element) => {
                        if element.click().await.is_ok() {
                            println!("Worker {} clicked element: {}", worker_id, selector);
                            result_tx.send(TaskResult {
                                id: task.id,
                                status: format!("Clicked element by Worker {}", worker_id),
                                details: Some(selector.clone()),
                            }).await.unwrap();
                        } else {
                            eprintln!("Worker {}: Failed to click element: {}", worker_id, selector);
                        }
                    }
                    Err(e) => {
                        eprintln!("Worker {}: Element not found: {}", worker_id, selector);
                        result_tx.send(TaskResult {
                            id: task.id,
                            status: format!(
                                "Failed to find element by Worker {}: {:?}",
                                worker_id, e
                            ),
                            details: None,
                        }).await.unwrap();
                    }
                }
            }
            _ => {
                eprintln!("Worker {} received unsupported task: {:?}", worker_id, task);
                result_tx.send(TaskResult {
                    id: task.id,
                    status: format!("Unsupported task type by Worker {}", worker_id),
                    details: None,
                }).await.unwrap();
            }
        }
    }

    println!("Worker {} exiting", worker_id);
    client.close().await.unwrap();
}
