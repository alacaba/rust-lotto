extern crate rand;

use std::collections::HashMap;
use std::io;
use rand::prelude::*;

fn main() {
    let mut lotto_modes = HashMap::new();
    let mut game_mode = String::new();
    let mut set = String::new();

    lotto_modes.insert(1, 58);
    lotto_modes.insert(2, 55);
    lotto_modes.insert(3, 49);
    lotto_modes.insert(4, 45);
    lotto_modes.insert(5, 42);

    println!("Select lotto mode: ");

    println!("1.) 6/58");
    println!("2.) 6/55");
    println!("3.) 6/49");
    println!("4.) 6/45");
    println!("5.) 6/42");
    println!("");

    io::stdin().read_line(&mut game_mode)
        .expect("Failed to readline");

    println!("gm: {}", &game_mode);

    let game_mode: i32 = game_mode.trim().parse()
        .expect("Please type a number");

    let mode = lotto_modes.get(&game_mode)
        .expect("Invalid game mode");

    println!("How many set of numbers do you want to generate: ");
    println!("");

    io::stdin().read_line(&mut set)
        .expect("Failed to readline");

    let set: i32 = set.trim().parse()
        .expect("Please type a number");

    for number in generate_numbers(&set, mode).iter() {
        println!("{:?}", number)
    }
}

fn generate_numbers(set: &i32, mode: &i32) -> Vec<Vec<i32>> {
    let mut numbers = Vec::new();
    let mut rng = thread_rng();

    for _ in 1..set+1 {
        let mut stack = Vec::new();
        for _ in 1..7  {
            let mut number = rng.gen_range(1, *mode);
            while has_duplicate(&stack, &number) {
                number = rng.gen_range(1, *mode);
            }
            stack.push(number)
        }
        numbers.push(stack)
    }

    numbers
}

fn has_duplicate(stack: &Vec<i32>, number: &i32) -> bool {
    let idx = match stack.iter().position(|&r| r == *number) {
        Some(n) => n,
        None => 0
    };

    idx != 0
}
