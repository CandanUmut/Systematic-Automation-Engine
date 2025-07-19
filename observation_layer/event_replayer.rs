pub async fn replay_events(client: &mut Client, recorder: &EventRecorder) {
    for event in recorder.get_events() {
        match event.action.as_str() {
            "click" => {
                let locator = Locator::Css(&event.target);
                client.wait_for_find(locator).await.unwrap().click().await.unwrap();
                println!("Replayed click on: {}", event.target);
            }
            "type" => {
                let locator = Locator::Css(&event.target);
                if let Some(value) = &event.value {
                    client.wait_for_find(locator).await.unwrap().send_keys(value).await.unwrap();
                    println!("Replayed typing on: {} with value: {}", event.target, value);
                }
            }
            _ => println!("Unknown action: {}", event.action),
        }
    }
}
