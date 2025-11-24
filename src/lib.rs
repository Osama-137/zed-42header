use chrono::Utc;

pub fn generate_42_header(file_name: &str) -> String {
    let current_time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    format!(
        "42 header for {} - Generated on: {}",
        file_name, current_time
    )
}

fn main() {
    // Entry point for Zed command: prints the header to stdout
    let file_name = std::env::args().nth(1).unwrap_or_else(|| "unknown.c".to_string());
    let header = generate_42_header(&file_name);
    println!("{}", header);
}