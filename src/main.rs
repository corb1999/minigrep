/* 
use std::env::args; // dont need to import some std lib calls since called explicetly just once or so
use std::process::exit; 
use std::fs::read_to_string;
// */
use minigrep::Config;

fn main() {
    println!("\nHello, I am minigrep! \n");

    let args: Vec<String> = std::env::args().collect();

    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            std::process::exit(1); 
        }); 

//    println!("{:?} \n", args);
    println!("  Searching for... '{}' \n", config.query); 
    println!("  Within file... '{}' \n", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e); 
        std::process::exit(1); 
    }

}



