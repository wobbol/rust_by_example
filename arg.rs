use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("path: {}", args[0]);
    println!("{:?} arguments: {:?}", args.len() - 1, &args[1..]);
}
