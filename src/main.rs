use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).unwrap();
    let filename = args.get(2).unwrap();
    println!("Searching for {query}");
    println!("In file {filename}");
}
