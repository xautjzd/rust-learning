use std::env;

fn main() {
    let shell = "SHELL";
    match env::var(shell) {
        Ok(x) => println!("you are using shell: {}", x),
        Err(e) => panic!("env: {} is unset yeat, {}", shell, e),
    }

    let s = env::var("TERM").expect("TERM env is unset");
    println!("env TERM is: {}", s);

    let go_root = env::var_os("GOROOT").expect("GOROOT env is unset");
    println!("env GOROOT is: {:?}", go_root);

    let zsh = env!("ZSH", "env ZSH is unset.");
    println!("env ZSH is: {}", zsh);

    let tmux = option_env!("TMUX");
    match tmux {
        Some(x) => println!("env TMUX is: {}", x),
        None => println!("env TMUX is unset."),
    }

    for (name, value) in env::vars() {
        println!("{}: {}", name, value);
    }
}
