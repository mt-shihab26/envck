use std::{collections::HashMap, fs::read_to_string, io::Result, process::exit};

fn get_file_content(file_name: &str) -> Result<Vec<String>> {
    let content = read_to_string(file_name)?;
    let file_content: Vec<String> = content.lines().map(String::from).collect();
    Ok(file_content)
}

fn get_file_map(file_content: Vec<String>) -> (HashMap<String, String>, Vec<String>) {
    let mut file_map = HashMap::new();
    let mut null_keys = Vec::new();
    for line in &file_content {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue;
        }
        if trimmed_line.contains("=") {
            if let Some((key, value)) = trimmed_line.split_once("=") {
                file_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        } else {
            null_keys.push(trimmed_line.to_string());
        }
    }
    (file_map, null_keys)
}

fn main() {
    let dotenv_file_name = ".env";
    let example_file_name = ".env.example";

    let dotenv_file_content = get_file_content(dotenv_file_name)
        .expect(&format!("Can't read the file '{dotenv_file_name}'"));
    let example_file_content = get_file_content(example_file_name)
        .expect(&format!("Can't read the file '{example_file_name}'"));

    let (dotenv_file_map, dotenv_null_keys) = get_file_map(dotenv_file_content);
    let (example_file_map, example_null_keys) = get_file_map(example_file_content);

    let missing_keys: Vec<&String> = example_file_map
        .keys()
        .filter(|key| !dotenv_file_map.contains_key(*key))
        .collect();
    let extra_keys: Vec<&String> = dotenv_file_map
        .keys()
        .filter(|key| !example_file_map.contains_key(*key))
        .collect();
    let dotenv_empty_keys: Vec<&String> = dotenv_file_map
        .iter()
        .filter(|(_, value)| value.is_empty())
        .map(|(key, _)| key)
        .collect();
    let example_empty_keys: Vec<&String> = example_file_map
        .iter()
        .filter(|(_, value)| value.is_empty())
        .map(|(key, _)| key)
        .collect();

    println!("--- stats ---");
    println!("{dotenv_file_name}: {dotenv_file_map:?}");
    println!("{example_file_name}: {example_file_map:?}");
    println!("{dotenv_file_name} key count: {}", dotenv_file_map.len());
    println!("{example_file_name} key count: {}", example_file_map.len());
    println!("missing keys (in {example_file_name}, not in {dotenv_file_name}): {missing_keys:?}");
    println!("extra keys (in {dotenv_file_name}, not in {example_file_name}): {extra_keys:?}");
    println!(
        "{dotenv_file_name} empty value key count: {} {dotenv_empty_keys:?}",
        dotenv_empty_keys.len()
    );
    println!(
        "{example_file_name} empty value key count: {} {example_empty_keys:?}",
        example_empty_keys.len()
    );
    println!(
        "{dotenv_file_name} null key count: {} {dotenv_null_keys:?}",
        dotenv_null_keys.len()
    );
    println!(
        "{example_file_name} null key count: {} {example_null_keys:?}",
        example_null_keys.len()
    );
    println!("-------------");

    if !missing_keys.is_empty() {
        println!(".env.example has more keys then .env. not matched");
        exit(1);
    }

    if !extra_keys.is_empty() {
        println!(".env has more keys then .env.example. not matched");
        exit(1);
    }

    println!(".env and .env.example key is match completely");
}
