use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: \n{error}");
        process::exit(1);
    });

    let (query, file_path) = (&config.query, &config.file_path);
    println!("In file {file_path} looking for {query}");
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(
                "Not enough arguments provided.\n\tUsage: cargo run -- <query> <file_path>",
            );
        }
        Ok(Config::new(args))
    }

    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
