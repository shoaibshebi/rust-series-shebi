use std::{io, cmp::Ordering};

use rand::Rng;



fn main() {
    println!("Guessing a game");

loop {
    

    let mut guess = String::new();

    let rand_input = rand::thread_rng().gen_range(1..=100);

    println!("rand input: {}", rand_input);

    io::stdin().read_line(&mut guess);

    let guess:u32 = guess.trim().parse().expect("failed to trim or parse");
    
    match guess.cmp(&rand_input) {
        Ordering::Less => println!("The guess is less"),
        Ordering::Greater => println!("The guess is greater"),
        Ordering::Equal =>{
            println!("The guess is equel");
            break;
        },
    }
    }

}

// implement loop



// Blocking waiting for file lock on package cache
    //rm -rf ~/.cargo/registry/index/*
    //rm -rf ~/.cargo/.package-cache
    //cargo run

//cargo install cargo-watch
// cargo-watch -qc -x run -x clippy

//rustup component add clippy
//rustup component add rustfmt