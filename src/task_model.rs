pub struct AutomationTask {
    pub steps: Vec<TaskStep>,
}

#[derive(Debug, Clone)]
pub struct Subtask {
    pub id: usize,
    pub description: String,
}

#[derive(Debug)]
pub struct TaskResult {
    pub id: usize,
    pub status: String,
}


impl AutomationTask {
    // Add a constructor for creating tasks
    pub fn new() -> Self {
        AutomationTask { steps: Vec::new() }
    }

    // Add a method to append steps
    pub fn add_step(&mut self, step: TaskStep) {
        self.steps.push(step);
    }
}

pub enum TaskStep {
    OpenWebsite(String),         // Open a URL
    ClickElement(String),        // Click an element by selector
    FillForm(String, String),    // Fill a form field (selector, value)
    WaitForElement(String),      // Wait for an element to appear
    TakeScreenshot(String),      // Take a screenshot and save to path
}
