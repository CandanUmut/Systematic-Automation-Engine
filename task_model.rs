use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationTask {
    pub steps: Vec<TaskStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subtask {
    pub id: usize,
    pub description: String,
    pub step: TaskStep, // Associate a specific task step with the subtask
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskResult {
    pub id: usize,
    pub status: String,
    pub details: Option<String>, // Optional details or result data
}

impl AutomationTask {
    /// Create a new automation task
    pub fn new() -> Self {
        AutomationTask { steps: Vec::new() }
    }

    /// Add a step to the automation task
    pub fn add_step(&mut self, step: TaskStep) {
        self.steps.push(step);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStep {
    OpenWebsite(String),         // Open a URL
    ClickElement(String),        // Click an element by selector
    FillForm(String, String),    // Fill a form field (selector, value)
    WaitForElement(String),      // Wait for an element to appear
    TakeScreenshot(String),      // Take a screenshot and save to path
    CategorizeElements,          // New: Categorize all elements on the page
}

impl TaskStep {
    /// Utility to create a subtask directly from a TaskStep
    pub fn to_subtask(self, id: usize) -> Subtask {
        Subtask {
            id,
            description: format!("{:?}", self),
            step: self,
        }
    }
}
