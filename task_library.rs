use std::fs::{self, File};
use std::io::{self, Write};

pub fn save_task(name: String, steps: String) {
    let file_name = format!("tasks/{}.txt", name);
    fs::create_dir_all("tasks").unwrap();
    let mut file = File::create(file_name).unwrap();
    writeln!(file, "{}", steps).unwrap();
}

pub fn load_task(name: String) -> Option<String> {
    let file_name = format!("tasks/{}.txt", name);
    match fs::read_to_string(file_name) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}
