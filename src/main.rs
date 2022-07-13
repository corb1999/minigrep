// use std::env::args; // not needed to import

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

    let contents = std::fs::read_to_string(config.filename)
        .expect("Something went wrong trying to read the file"); 

    println!("Which has text of... \n '{}'", contents);

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

