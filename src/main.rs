/* 
use std::env::args; // dont need to import some std lib calls since called explicetly just once or so
use std::process::exit; 
use std::fs::read_to_string;
// */
use std::error::Error;

fn main() {
    println!("\n Hello, I am minigrep! \n");

    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            std::process::exit(1); 
        }); 

    println!("{:?} \n", args);
    println!("Searching for... '{}' \n", config.query); 
    println!("Within file... '{}' \n", config.filename);
    
    // run(config); 

    if let Err(e) = run(config) {
        println!("Application error: {}", e); 
        std::process::exit(1); 
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?;
        
    println!("Which has text of...\n'{}'", contents);

    Ok(())
}

struct Config {
    query: String, 
    filename: String, 
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("***Not enough arguments"); 
        }

        let query = args[1].clone(); 
        let filename = args[2].clone(); 

        Ok(Config { query, filename })
    }
}

