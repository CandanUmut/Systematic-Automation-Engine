pub struct InteractionLog {
    pub action: String,
    pub selector: String,
    pub status: String,
    pub timestamp: String,
}

impl InteractionLog {
    pub fn new(action: &str, selector: &str, status: &str) -> Self {
        Self {
            action: action.to_string(),
            selector: selector.to_string(),
            status: status.to_string(),
            timestamp: chrono::Local::now().to_rfc3339(),
        }
    }

    pub fn log(&self) {
        println!(
            "[{}] Action: {}, Selector: {}, Status: {}",
            self.timestamp, self.action, self.selector, self.status
        );
    }
}
