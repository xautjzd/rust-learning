use std::io;

fn main() {
    let mut result = String::new();
    match io::stdin().read_line(&mut result) {
        Ok(x) => {
            println!("received from stdin is: {}, size is: {}", result, x);
        }
        Err(e) => panic!("read error: {}", e),
    }
}
