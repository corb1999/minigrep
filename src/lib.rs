use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = std::fs::read_to_string(config.filename)?; 
    println!("  Which has text of...\n'{}'\n*END OF TEXT\n", contents);

    let results = if config.ignore_case {
        search_case_insentive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    }; 

    println!("  Lines with a found match..."); 
    for line in results {
        println!("{}", line); 
    }

    Ok(())
}

// original search fn written in ch 12
/*pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new(); 
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line); 
        }
    }

    results
}*/

// rewriting search fn per ch 13.3 to use iterator adaptors instead of a loop
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insentive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); 
    let mut results = Vec::new(); 

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line); 
        }
    }
    
    results
}

pub struct Config {
    pub query: String, 
    pub filename: String, 
    pub ignore_case: bool, 
}

impl Config {
    // this is the original function built in ch 12, but I never renamed it build
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("***Not enough arguments"); 
        }

        let query = args[1].clone(); 
        let filename = args[2].clone(); 

        let ignore_case = std::env::var("IGNORE_CASE").is_ok(); 

        Ok(Config { query, filename, ignore_case, })
    }

    // suggested new fn using iterators per ch 13.3
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); 

        let query = match args.next() {
            Some(arg) => arg, 
            None => return Err("No query string"), 
        };

        let filename = match args.next() {
            Some(arg) => arg, 
            None => return Err("No file path"),  
        };

        let ignore_case = std::env::var("IGNORE_CASE").is_ok(); 

        Ok(Config { query, filename, ignore_case} )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result_case_sensitive() {
        let query = "duct"; 
        let contents = "/
Rust: 
safe, fast, productive.
Pick three.
Duct tape is an option.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); 
    }

    #[test]
    fn two_result_case_insensitive() {
        let query = "rUsT"; 
        let contents = "\
Rust: 
safe, fast, productive.
Pick three. 
Trust me.";
        
        assert_eq!(vec!["Rust: ", "Trust me."], search_case_insentive(query, contents)); 
    }

}