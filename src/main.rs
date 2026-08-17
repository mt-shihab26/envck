use std::{collections::HashMap, fs::read_to_string, io::Result};

fn get_file_content(file_name: &str) -> Result<Vec<String>> {
    let content = read_to_string(file_name)?;
    let file_content: Vec<String> = content.lines().map(String::from).collect();
    Ok(file_content)
}

fn get_file_map(file_content: Vec<String>) -> HashMap<String, String> {
    let mut file_map = HashMap::new();
    for line in &file_content {
        if line.trim().contains("=") {
            if let Some((key, value)) = line.split_once("=") {
                file_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
    }
    file_map
}

fn main() {
    let dotenv_file_name = ".env";
    let example_file_name = ".env.example";

    let dotenv_file_content = get_file_content(dotenv_file_name)
        .expect(&format!("Can't read the file '{dotenv_file_name}'"));
    let example_file_content = get_file_content(example_file_name)
        .expect(&format!("Can't read the file '{example_file_name}'"));

    let dotenv_file_map = get_file_map(dotenv_file_content);
    let example_file_map = get_file_map(example_file_content);

    println!("{dotenv_file_map:?}");
    println!("{example_file_map:?}");

    for (example_key, example_value) in example_file_map {
        let dotenv_key = &dotenv_file_map[&example_key];
    }

    println!("all required env vars are present");
}
