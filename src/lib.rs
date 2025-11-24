/// Generates a 42 header with the current date and time stamps.
///
/// # Example Usage
/// ```
/// let header = generate_42_header("my_file.rs");
/// println!("{}", header);
/// ```
///
/// # Returns
/// The formatted 42 header string.
fn generate_42_header(file_name: &str) -> String {
    let current_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    format!("42 header for {} - Generated on: {}", file_name, current_time)
}

fn main() {
    // Example of generating a 42 header
    let header = generate_42_header("example.rs");
    println!("{}", header);
}