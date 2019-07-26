use std::env; 
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing args :{}",err );
        process::exit(1);
    });
   
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);

}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

fn run(config:Config) {

    let contents = fs::read_to_string(config.filename)
        .expect("read uncorrectly");
    println!("content:\n{}",contents );
}