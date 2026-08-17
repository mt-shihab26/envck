use std::{collections::HashMap, fs::read_to_string, io::Result, process::exit};

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

    for (example_key, _example_value) in &example_file_map {
        let dotenv_value = dotenv_file_map.get(example_key);
        if let None = dotenv_value {
            println!("{dotenv_file_map:?}");
            println!("{example_file_map:?}");
            println!(".env.example has more keys then .env. not matched");
            exit(1);
        }
    }

    for (dotenv_key, _donenv_value) in &dotenv_file_map {
        let example_value = example_file_map.get(dotenv_key);
        if let None = example_value {
            println!("{dotenv_file_map:?}");
            println!("{example_file_map:?}");
            println!(".env has more keys then .env.example. not matched");
            exit(1);
        }
    }

    println!(".env and .env.example key is match completely");
}
