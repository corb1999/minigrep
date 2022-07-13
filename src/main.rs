// use std::env::args; // not needed to import

fn main() {
    println!("\n Hello, I am minigrep! \n");

    let args: Vec<String> = std::env::args().collect();

    let config = parse_config(&args); 

    println!("{:?}", args);
    println!("Searching for... '{}'", config.query); 
    println!("Within file... '{}'", config.filename);

    let contents = std::fs::read_to_string(config.filename)
        .expect("Something went wrong trying to read the file"); 

    println!("Which has text of... \n{}", contents);

}

struct Config {
    query: String, 
    filename: String, 
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone(); 

    Config { query, filename }
}
