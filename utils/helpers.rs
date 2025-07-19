pub fn format_timestamp() -> String {
    chrono::Local::now().to_rfc3339()
}

pub fn capitalize_first_letter(text: &str) -> String {
    let mut c = text.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
