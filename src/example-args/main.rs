use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("args size: {}", args.len());

    if args.len() > 1 {
        println!("first argment is: {}", args[1]);
    }

    for argument in env::args() {
        println!("{argument}");
    }
}
