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
    let content = fs::read_to_string(&config.filename)?;

    if config.operation == "search" {
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
        println!("Replacing {} with {}", config.expression, config.filename);
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