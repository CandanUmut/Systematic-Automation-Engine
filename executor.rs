use crate::web_interaction::*;
use fantoccini::Client;
use crate::task_model::{AutomationTask, TaskStep};

pub async fn execute_task(task: AutomationTask) {
    println!("🚀 Starting task execution...");

    let mut client = match start_browser().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("❌ Failed to start browser: {:?}", err);
            return;
        }
    };

    for step in task.steps {
        match step {
            TaskStep::OpenWebsite(url) => {
                if let Err(err) = open_website(&mut client, &url).await {
                    eprintln!("❌ Failed to open website {}: {:?}", url, err);
                }
            }
            TaskStep::ClickElement(selector) => {
                if let Err(err) = click_element(&mut client, &selector).await {
                    eprintln!("❌ Failed to click element {}: {:?}", selector, err);
                }
            }
            TaskStep::FillForm(selector, value) => {
                if let Err(err) = fill_form_field(&mut client, &selector, &value).await {
                    eprintln!("❌ Failed to fill form field {}: {:?}", selector, err);
                }
            }
            TaskStep::TakeScreenshot(file_path) => {
                if let Err(err) = take_screenshot(&mut client, &file_path).await {
                    eprintln!("❌ Failed to take screenshot: {:?}", err);
                }
            }
            TaskStep::WaitForElement(selector) => {
                if let Err(err) = wait_for_element(&mut client, &selector).await {
                    eprintln!("❌ Failed to wait for element {}: {:?}", selector, err);
                }
            }
            TaskStep::CategorizeElements => {
                match universal_locator(&mut client).await {
                    Ok(categorized_elements) => {
                        println!("✅ Categorized elements: {:?}", categorized_elements);
                    }
                    Err(err) => {
                        eprintln!("❌ Failed to categorize elements: {:?}", err);
                    }
                }
            }
        }
    }

    if let Err(err) = client.close().await {
        eprintln!("❌ Failed to close browser: {:?}", err);
    } else {
        println!("✅ Task execution completed!");
    }
}
