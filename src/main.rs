// use std::env::args; // not needed to import

fn main() {
    println!("\n Hello, I am minigrep! \n");

    let args: Vec<String> = std::env::args().collect();

    let query = &args[1];
    let filename = &args[2]; 

    let contents = std::fs::read_to_string(filename)
        .expect("Something went wrong trying to read the file"); 

    println!("{:?}", args);
    println!("Searching for... '{}'", query); 
    println!("Within file... '{}'", filename); 
    println!("Which has text of... \n{}", contents);

}
