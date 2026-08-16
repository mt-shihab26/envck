use std::fs::read_to_string;

fn main() {
    let dotenv_file_name = ".env";
    let example_file_name = ".env.example";

    let dotenv_file_content = read_to_string(dotenv_file_name)
        .expect(&format!("Can't read the file '{dotenv_file_name}'"));
    let example_file_content = read_to_string(example_file_name)
        .expect(&format!("Can't read the file '{example_file_name}'"));

    let dotenv_file_content: Vec<&str> = dotenv_file_content.split("\n").collect();
    let example_file_content: Vec<&str> = example_file_content.split("\n").collect();

    println!("{:?}", dotenv_file_content);
    println!("{:?}", example_file_content);
}
