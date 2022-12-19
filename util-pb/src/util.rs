pub fn get_summary(content: &str) -> String {
    if content.len() <= 255 {
        return String::from(content);
    }
    content.chars().into_iter().take(255).collect()
}
