//use std::io;
fn main() {
    println!("Hello, world!");
    println!("{} bytes.", std::mem::size_of::<usize>());

    let mut a = 100;
    let mut b: u32 = 100;
    a = a * (-1);
    println!("A = {}", a);
    println!("B = {}", b);

    let mut f = 3.141592;
    println!("F = {}", f);
    println!("{}", std::mem::size_of_val(&f));
    println!("i32 == {} bytes.", std::mem::size_of::<i32>());
    f *= f;
    println!("F = {}", f);
    let a = 200; // ! Shadowing int.
    println!("Shadow A = {}", a);
    println!("i16 Max Value = {}", i16::MAX);
    println!("i8 Max Value = {}", i8::MAX);

    // ?    let x:i8 = 128;
    // ?    println!("{}",x); giving error.

    let boolean = true;
    println!("{}", std::mem::size_of_val(&boolean));

    print!("Hello\n");
    print!("Hello\n\n");

    // ! Operations & Loops From File-3/2nd vid.
    println!("{}", a + b);
    println!("{}", a - b);
    println!("{}", a * b);
    println!("{}", a % b);
    println!("{}", a / b);
    println!();

    // ! Loops
    println!("For-Loops:");
    for i in 1..=5 {
        println!("I = {}", i);
    }
    println!();
    println!("While-Loops:");
    let mut i: i8 = 0;
    while i < 5 {
        println!("I = {}", i);
        i += 1;
    }
    println!();
    if i != 5 {
        println!("Nice Nice...!!");
    } else if i == 5 {
        println!("New Nice");
    }
    println!();
    println!("Loop:");
    i = 0;
    loop {
        print!("{}  ", i);
        i += 1;
        if i == 5 {
            break;
        }
    }
    println!();
    println!();

    // ! Array
    println!();
    println!("Array:");
    let mut arr: [i32; 5] = [5, 6, 7, 8, 9];
    let zero: [i8; 3] = [0; 3];

    println!("{:?}", zero);
    println!();

    println!("Address:");
    println!("{:#?}", arr.as_ptr());
    println!();

    println!("Normal Access:");
    for i in 0..arr.len() {
        print!("{}  ", arr[i]); //? unsafe!!
    }
    println!();
    println!();

    println!("Safe Access:");
    for i in 0..=arr.len() {
        let r = arr.get(i); // ! arr.get(100);
        match r {
            Some(value) => print!("{}  ", value),
            None => {}
        };
    }
    println!();
    println!();

    println!("Iterator Access:");
    for i in arr.iter() {
        println!("{} => {}", i, *i);
    }
    println!();

    println!("Mutable Iterator Access:");
    for i in arr.iter_mut() {
        *i += *i;
        print!("{}  ", i);
        *i /= 2;
    }
    println!();
    println!();

    println!("Into Iterator Access:");
    for i in arr.into_iter() {
        print!("{}  ", i);
    }
    println!();
    println!();

    println!("Direct Access:");
    println!("{:?}", arr);
    println!();
    println!("Direct Access Also Use for Checking:");
    println!("5 Exists = {:?}", arr.contains(&5));
    println!("10 Exists = {:?}", arr.contains(&10));
    println!();

    println!("Vectors:");
    let v = arr.iter().map(|x| x * x).collect::<Vec<i32>>();
    println!("{:?} => {:#?}", arr, v);
    // ! {:?} => Use for lenior array {:#?} => Use for vertical array.

    /* //! Input...
    let mut input = String::new();
    println!("Enter an integer:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("{}",input);
     */

    // ! Vector
    let mut v: Vec<i32> = vec![10, 20, 30, 40, 50];
    println!("Vector v =  {:?}", v);

    let mut v1: Vec<i32> = Vec::new();
    println!("Vector v1 = {:?}", v1);

    for _ in 0..v.len() {
        v1.push(v.pop().unwrap());
    }
    println!("Vector v1 = {:?}", v1);
    println!("Vector v =  {:?}", v);

    for i in v1.clone().into_iter() {
        print!("{}  ", i);
    }
    println!();

    let sum: i32 = v1.iter().sum();
    println!("Sum of v1 = {}", sum);

    v1 = v1.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("Vector v1 = {:?}", v1);

    v = (1..=5).collect::<Vec<i32>>();
    println!("Vector v =  {:?}", v);
    println!();

    println!("Matrix:");
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut k = 1;
    for _ in 0..3 {
        let mut row: Vec<i32> = Vec::new();
        for _ in 0..3 {
            row.push(k);
            k += 1;
        }
        matrix.push(row);
    }
    matrix.push(v);
    matrix.push(v1);
    println!("{:?}", matrix);
    for i in 0..5 {
        for j in 0..3 {
            print!("{}  ", matrix[i][j]);
        }
        println!();
    }
    println!("{}", matrix[1][2]);
    println!();

    // ! Char x String.
    let name = "Morningstar_2061";
    println!("{}", name);
    println!("{:?}", name);

    for i in name.chars() {
        print!("{}  ", i);
    }
    println!();

    let mut newname: String = name.to_string();
    println!("{:?}", newname);

    newname.clear();
    newname.push_str("Hello ");
    newname.push_str(name);
    println!("{:?}", newname);

    // ! ASCII.
    let ascii: Vec<u8> = vec![65, 66, 67, 68, 69, 70];
    println!("{}", String::from_utf8_lossy(&ascii[..]));

    b = 5;
    let c = 10;

    println!("B = {}", b);
    println!("C = {}", c);

    let c = &b;
    println!("C = {}", *c);

    b += *c;
    println!("B = {}", b);

    let s = String::from(name);
    let mut s2 = s.clone();
    let s3 = &s;

    println!("S2 = {:?}", s2);
    s2 = funname(s2);

    println!("S = {:?}", s);
    // drop(s);
    // s.clear();
    println!("New S2 = {:?}", s2);
    println!("S3 = {:?}", s3);

    // ! HashMap
    use std::collections::HashMap;
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert(
        String::from("Ancient Roman History"),
        String::from("Very accurate."),
    );
    reviews.insert(
        String::from("Cooking with Rhubarb"),
        String::from("Sweet recipes."),
    );
    reviews.insert(
        String::from("Programming in Rust"),
        String::from("Great examples."),
    );

    // Look for a specific review
    let book: &str = "Programming in Rust";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    // Remove book review
    let obsolete: &str = "Ancient Roman History";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    // Confirm book review removed
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}

fn funname(mut name: String) -> String {
    name.push_str(" Funtion.");
    return name;
}
