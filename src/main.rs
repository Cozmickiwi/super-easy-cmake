use secm2::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match build_main(&args[args.len() - 1]) {
        Ok(_) => println!("C project created successfully!"),
        Err(e) => println!("Error creating project: {e}"),
    }
}
