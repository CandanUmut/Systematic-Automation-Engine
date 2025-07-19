use super::interaction_log::InteractionLog;

pub struct LogRepository {
    logs: Vec<InteractionLog>,
}

impl LogRepository {
    pub fn new() -> Self {
        Self { logs: vec![] }
    }

    pub fn add_log(&mut self, log: InteractionLog) {
        self.logs.push(log);
        log.log(); // Print log details for immediate feedback
    }

    pub fn view_logs(&self) {
        println!("--- Logs ---");
        for log in &self.logs {
            log.log();
        }
    }
}
