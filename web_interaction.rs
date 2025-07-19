use fantoccini::{Client, Locator};
use std::error::Error;
use std::fs;
use anyhow::{Result, anyhow};
use thiserror::Error;
use fantoccini::error::CmdError;
use std::collections::HashMap;

#[derive(Error, Debug)]
pub enum WebInteractionError {
    #[error("Element did not appear in time: {0}")]
    ElementNotFound(String),
    #[error("Fantoccini error: {0}")]
    FantocciniError(#[from] CmdError),
}

pub async fn start_browser() -> Result<Client, fantoccini::error::NewSessionError> {
    println!("Starting browser for automation...");
    let client = Client::new("http://localhost:4444").await?;
    Ok(client)
}

pub async fn open_website(client: &mut Client, url: &str) -> Result<(), fantoccini::error::CmdError> {
    println!("🌐 Navigating to: {}", url);
    client.goto(url).await
}

pub async fn click_element(client: &mut Client, selector: &str) -> Result<(), fantoccini::error::CmdError> {
    println!("🖱️ Clicking element: {}", selector);
    let element = client.find(Locator::Css(selector)).await?;
    element.click().await.map(|_| ()) // Convert the Result<Client, CmdError> to Result<(), CmdError>
}


pub async fn fill_form_field(client: &mut Client, selector: &str, value: &str) -> Result<(), fantoccini::error::CmdError> {
    println!("⌨️ Filling form field: {} with {}", selector, value);
    let mut element = client.find(Locator::Css(selector)).await?; // Declare as mutable
    element.send_keys(value).await
}


pub async fn take_screenshot(client: &mut Client, file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("📸 Taking screenshot and saving to: {}", file_path);
    let png_data = client.screenshot().await?;
    fs::write(file_path, png_data)?;
    Ok(())
}

pub async fn wait_for_element(
    client: &mut Client,
    selector: &str,
) -> Result<()> {
    println!("⏳ Waiting for element to appear: {}", selector);
    for _ in 0..30 {
        if client.find(Locator::Css(selector)).await.is_ok() {
            println!("✅ Element appeared: {}", selector);
            return Ok(());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    Err(anyhow!("Element '{}' did not appear in time", selector))
}

pub async fn wait_for_element_with_timeout(
    client: &mut Client,
    selector: &str,
    timeout_secs: u64,
) -> Result<()> {
    println!("⏳ Waiting for element to appear: {}", selector);
    for _ in 0..timeout_secs {
        if client.find(Locator::Css(selector)).await.is_ok() {
            println!("✅ Element appeared: {}", selector);
            return Ok(());
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    Err(anyhow!(
        "Element '{}' did not appear in {} seconds",
        selector,
        timeout_secs
    ))
}


use std::time::Duration;
use std::thread::sleep;

pub async fn universal_locator(client: &mut Client) -> Result<HashMap<String, Vec<HashMap<String, String>>>, fantoccini::error::CmdError> {
    println!("🔍 Locating and categorizing all elements...");

    let categories = vec![
        ("Buttons", "button"),
        ("Inputs", "input"),
        ("Text Areas", "textarea"),
        ("Links", "a"),
        ("Divs", "div"),
        ("Images", "img"),
        ("Headers", "h1, h2, h3, h4, h5, h6"),
        ("Paragraphs", "p"),
        ("Spans", "span"),
    ];

    let mut categorized_elements: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();

    for (category, selector) in categories {
        println!("🔍 Searching for {} using selector '{}'", category, selector);
        // sleep(Duration::from_secs(2)); // Pause for observation

        let mut elements_data = vec![];

        match client.find_all(Locator::Css(selector)).await {
            Ok(elements) => {
                println!("🔍 Found {} elements for category '{}'", elements.len(), category);
                for mut element in elements {
                    // sleep(Duration::from_secs(1)); // Pause before processing each element
                    let mut element_data = HashMap::new();

                    if let Ok(Some(id)) = element.attr("id").await {
                        element_data.insert("id".to_string(), id);
                    } else {
                        element_data.insert("id".to_string(), "N/A".to_string());
                    }

                    if let Ok(Some(class)) = element.attr("class").await {
                        element_data.insert("class".to_string(), class);
                    } else {
                        element_data.insert("class".to_string(), "N/A".to_string());
                    }

                    if let Ok(Some(name)) = element.attr("name").await {
                        element_data.insert("name".to_string(), name);
                    } else {
                        element_data.insert("name".to_string(), "N/A".to_string());
                    }

                    // Execute JavaScript for additional properties
                    if let Ok(tag_name) = client
                        .execute(
                            &format!(
                                "return (function() {{
                                    let el = document.querySelector('{}');
                                    return el ? el.tagName : null;
                                }})();",
                                selector
                            ),
                            vec![],
                        )
                        .await
                    {
                        if let Some(tag_name_str) = tag_name.as_str() {
                            element_data.insert("tag".to_string(), tag_name_str.to_string());
                        }
                    }

                    if let Ok(is_displayed) = client
                        .execute(
                            &format!(
                                "return (function() {{
                                    let el = document.querySelector('{}');
                                    return el ? (el.offsetWidth > 0 && el.offsetHeight > 0) : false;
                                }})();",
                                selector
                            ),
                            vec![],
                        )
                        .await
                    {
                        if let Some(displayed) = is_displayed.as_bool() {
                            element_data.insert("visible".to_string(), displayed.to_string());
                        }
                    }

                    if let Ok(is_enabled) = client
                        .execute(
                            &format!(
                                "return (function() {{
                                    let el = document.querySelector('{}');
                                    return el ? !el.disabled : false;
                                }})();",
                                selector
                            ),
                            vec![],
                        )
                        .await
                    {
                        if let Some(enabled) = is_enabled.as_bool() {
                            element_data.insert("enabled".to_string(), enabled.to_string());
                        }
                    }

                    elements_data.push(element_data);
                }
                categorized_elements.insert(category.to_string(), elements_data);
            }
            Err(e) => {
                println!("❌ Failed to locate elements for category '{}': {:?}", category, e);
            }
        }
    }

    println!("✅ Finished locating and categorizing elements.");
    Ok(categorized_elements)
}

