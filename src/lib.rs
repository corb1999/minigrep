use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?;
        
    println!("Which has text of...\n'{}'", contents);

    Ok(())
}

pub struct Config {
    pub query: String, 
    pub filename: String, 
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("***Not enough arguments"); 
        }

        let query = args[1].clone(); 
        let filename = args[2].clone(); 

        Ok(Config { query, filename })
    }
}