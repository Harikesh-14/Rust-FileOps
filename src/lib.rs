// use std::env;

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

pub fn run (config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Operation: {}", config.operation);
    println!("Expression: {}", config.expression);
    println!("Filename: {}", config.filename);
    println!("Is sensitive: {}", config.is_sensitive);

    Ok(())
    
}