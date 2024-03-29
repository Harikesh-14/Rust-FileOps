use std::io;
use std::fs;
use std::error::Error;
use std::fs::{create_dir, File, OpenOptions, remove_dir_all, remove_file};
use std::io::Write;

pub struct Config {
    pub operation: String,
    pub expression: String,
    pub filename: String,
    pub is_sensitive: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Arguments error")
        }

        let operation = args[1].clone();
        let expression = args.get(2).cloned().unwrap_or_default();
        let filename = args.get(3).cloned().unwrap_or_else(|| String::from("true"));
        let is_sensitive = args.get(4).cloned().unwrap_or_else(|| String::from("true"));

        Ok(Config { operation, expression, filename, is_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    match config.operation.as_str() {
        "search" => {
            let content = fs::read_to_string(&config.filename)?;
            println!("Searching for '{}'...", config.expression);
            let search_res = search_word(&config.expression, &content, &config.is_sensitive);
            if search_res.is_empty() {
                println!("No matched word");
            } else {
                for line in search_res {
                    println!("{}", line);
                }
            }
        }
        "replace" => {
            let mut content = fs::read_to_string(&config.filename)?;
            let replace_res = search_word(&config.expression, &content, &config.is_sensitive);
            if replace_res.is_empty() {
                println!("No matched word found");
            } else {
                println!("Replacing {} in {}", config.expression, config.filename);
                let mut replaced_word = String::new();
                println!("Enter the new word: ");
                io::stdin()
                    .read_line(&mut replaced_word)
                    .expect("Expected an input");
                let replaced_word = replaced_word.trim();
                replace_word(&config.expression, replaced_word, &mut content, &config.is_sensitive)?;
                fs::write(&config.filename, content)?;
                println!("Word replaced successfully");
            }
        }
        "write" => {
            if config.expression == "-a" {
                println!("Write here: ");
                let mut content = String::new();
                io::stdin()
                    .read_line(&mut content)
                    .expect("Expected an input");
                let content = content.trim().to_string();
                match write_in_file(&content, &config.filename) {
                    Ok(_) => println!("Successfully wrote to the file!"),
                    Err(err) => eprintln!("Error writing to file: {}", err),
                }
            } else {
                println!("Did you mean `cargo run write -a <filename>` ?");
            }
        }
        "create" => {
            if config.expression == "-a" {
                match create_file(&config.filename) {
                    Ok(_) => println!("Successfully created the file"),
                    Err(err) => println!("Error creating the file: {}", err),
                }
            } else {
                println!("Did you mean `cargo run create -a <filename>` ?");
            }
        }
        "delete" => {
            if config.expression == "-d" {
                match delete(&config.filename) {
                    Ok(_) => println!("Successfully deleted the file"),
                    Err(err) => println!("Error deleting the file: {}", err),
                }
            } else {
                println!("Did you mean `cargo run delete -d <filename>` ?");
            }
        }
        "newdir" => {
            if config.expression == "-a" {
                match create_directory(&config.filename) {
                    Ok(_) => println!("Directory {} has been created", config.filename),
                    Err(err) => println!("Failed creating the directory: {}", err),
                }
            } else {
                println!("Did you mean `cargo run delete -d <filename>` ?");
            }
        }
        "dldir" => {
            if config.expression == "-d" {
                match delete_directory(&config.filename) {
                    Ok(_) => println!("Deleted the directory {}", config.filename),
                    Err(err) => println!("Error creating a directory: {}", err),
                }
            } else {
                println!("Did you mean `cargo run dldir -d <filename>` ?");
            }
        }
        "show" => {
            match list_directory_content(&config.expression) {
                Ok(_) => println!("-x-x-x-x-x-x-x-x-x-x-x-x-x-"),
                Err(err) => println!("Error listing the content: {}", err),
            }
        }
        _ => {
            println!("Invalid operation");
        }
    }
    Ok(())
}

pub fn create_file (filename: &str) -> io::Result<File> {
    let file = File::create(filename)?;
    Ok(file)
}

pub fn create_directory (directory_name: &str) -> io::Result<()> {
    create_dir(directory_name)?;
    Ok(())
}

pub fn list_directory_content (dirname: &str) -> io::Result<()>{
    for entry in fs::read_dir(dirname)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();
        println!("{}", file_name);
    }

    Ok(())
}

pub fn search_word<'a> (word: &str, content: &'a str, is_sensitive: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    if is_sensitive == "true" {
        for line in content.lines() {
            if line.contains(word) {
                result.push(line.trim())
            }
        }
    } else if is_sensitive == "false" {
        let word = word.to_lowercase();
        for line in content.lines() {
            if line.to_lowercase().contains(&word) {
                result.push(line.trim())
            }
        }
    } else {
        eprintln!("Enter either true or false");
    }

    result
}

pub fn replace_word (word_to_replace: &str, replacement_word: &str, content: &mut String, is_case_sensitive: &str) -> io::Result<()> {
    if is_case_sensitive == "true" {
        *content = content.replace(word_to_replace, replacement_word);
    } else if is_case_sensitive == "false" {
        let word_to_replace_lower = word_to_replace.to_lowercase();
        let mut modified_content = String::new();
        for line in content.lines() {
            if line.to_lowercase().contains(&word_to_replace_lower) {
                modified_content.push_str(&line.replace(word_to_replace, replacement_word));
            } else {
                modified_content.push_str(line);
            }
            modified_content.push('\n');
        }
        *content = modified_content;
    } else {
        eprintln!("Enter either true [CASE SENSITIVE] or false [CASE INSENSITIVE]");
    }

    Ok(())
}

pub fn write_in_file(content: &String, filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)?;

    writeln!(file, "{}", content)?;

    println!("Successfully appended the file");
    Ok(())
}

pub fn delete (filename: &str) -> io::Result<()> {
    let file = remove_file(filename)?;
    Ok(file)
}

pub fn delete_directory (directory_name: &str) -> io::Result<()> {
    remove_dir_all(directory_name)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive_searching_function() {
        let word = "om";
        let content = "Om, Namah, Shivaay\nomicron";
        let is_sensitive = "true";
        assert_eq!(vec!["Om, Namah, Shivaay"], search_word(word, content, is_sensitive));
    }

    #[test]
    fn case_insensitive_searching_function() {
        let word = "om";
        let content = "Om, Namah, Shivaay\nom, namah, shivaay";
        let is_sensitive = "false";
        assert_eq!(vec!["om, namah, shivaay"], search_word(word, content, is_sensitive));
    }

    #[test]
    fn case_sensitive_replace_function() {
        let mut content = "Hello World\nThis is a test".to_string();
        let word_to_replace = "World";
        let replacement_word = "Rust";
        let is_sensitive = "true";

        replace_word(word_to_replace, replacement_word, &mut content, is_sensitive).unwrap();

        assert_eq!(content, "Hello Rust\nThis is a test");
    }

    #[test]
    fn case_insensitive_replace_function() {
        let mut content = "Hello World\nThis is a test".to_string();
        let word_to_replace = "World";
        let replacement_word = "Rust";
        let is_sensitive = "false";

        replace_word(word_to_replace, replacement_word, &mut content, is_sensitive).unwrap();

        assert_eq!(content, "Hello Rust\nThis is a test");
    }
}