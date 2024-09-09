use colored::*;
use std::io;

pub fn recap() {
    println!("\nRUST Recap.");

    print!("\nEnter a String: ");
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    let in_str = string_input();
    println!("Read String: {}", in_str);

    let num: i32 = i32_input();
    println!("Read Number: {}", num);

    println!("\nVariables:");
    let x = 10; // ! Mutable using mut
    println!("x: i32 : {}", x);

    const CONSTANT: i32 = 1_000; // ! Can't be mutable.
    println!("Constant : {}", CONSTANT);

    let x: u32 = 20; // ! Shadowing
    println!("Shadowing of x: u32 : {}", x);

    println!("\nTuples:");
    let tup = ("Morningstar", 2061);
    let (name, _year) = tup;
    let year = tup.1;

    println!("{} - {} - {}", name, year, tup.0);

    println!("\nArray:");
    let byte: [i8; 5] = [0; 5];
    for i in 0..byte.len() {
        print!("{:?} ", byte.get(i));
    }
    println!();
    for i in byte.iter() {
        print!("{} ", i);
    }
}

fn string_input() -> String {
    let mut in_str = String::new();
    io::stdin()
        .read_line(&mut in_str)
        .expect("Failed to read input");
    in_str.trim().to_string()
}

fn i32_input() -> i32 {
    loop {
        print!("\nEnter a Number: ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read input");

        let trim = num.trim();
        match trim.parse::<i32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("{}", "Please enter a valid number.".red());
            }
        }
    }
}
