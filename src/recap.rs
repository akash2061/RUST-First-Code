use std::io;

pub fn recap() {
    println!("\nRUST Recap.");

    print!("\nEnter a String: ");
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    let in_str = string_input();
    println!("Read String: {}", in_str);

    let num: i32 = i32_input();
    println!("Read Number: {}", num);
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
                println!("Please enter a valid number.");
            }
        }
    }
}
