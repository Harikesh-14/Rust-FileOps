use std::io;
use std::fs;
use std::error::Error;

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
        let expression = args[2].clone();
        let filename = args[3].clone();
        let is_sensitive = args[4].clone();

        Ok(Config { operation, expression, filename, is_sensitive })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    if config.operation == "search" {
        let content = fs::read_to_string(&config.filename)?;
        println!("Searching for '{}'...", config.expression);
        // implementing the searching function
        let search_res = search_word(&config.expression, &content, &config.is_sensitive);
        // printing the result
        if search_res.len() == 0 {
            println!("No matched word")
        } else {
            for line in search_res {
                println!("{}", line)
            }
        }

    } else if config.operation == "replace" {
        let mut content = fs::read_to_string(&config.filename)?;
        
        let replace_res = search_word(&config.expression, &content, &config.is_sensitive);

        if replace_res.len() == 0 {
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
    } else {
        println!("Invalid operation");        
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

}