use fantoccini::{Client, Locator};
use std::error::Error;
use std::fs;
use anyhow::{Result, anyhow};
use thiserror::Error;
use fantoccini::error::CmdError;

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