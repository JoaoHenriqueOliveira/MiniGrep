use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| { //* We customize our message if an error arises
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file {}", config.filename);

    //TODO: 
    run(config);

   
}

fn run (config: Config){
    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text: \n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();       //* Big runtime cost, but we accept for now
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone(); //trade-off to gain simplicity and loose performance
//     let filename = args[2].clone();

//     Config{query, filename}
// }

//! bom dia 

//TODO: Fazer isso
//? should this be mentioned?
//* Highlight!!






