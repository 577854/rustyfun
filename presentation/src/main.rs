fn main() {
  
    //part1();
    part2();
}

fn part1(){
    println!("--- Ownership Transfer ---");

    // `s1` owns the String
    let s1 = String::from("Hello, Rust!");

    // Ownership is transferred to `s2`
    let s2 = s1;

    //println!("{}", s1); //<--This will cause a compile error! `s1` was moved.

    println!("s2 = {}", s2);

    println!("\n--- Immutable Borrow ---");

    let name = String::from("Bob");

    // We borrow `name` without taking ownership
    print_name(&name);

    // We can still use `name` after an immutable borrow
    println!("name is still accessible: {}", name);

    println!("\n--- Mutable Borrow ---");

    let mut message = String::from("Hi");

    // Mutable borrow lets us change the original value
    append_world(&mut message);

    // We can use it again after the borrow ends
    println!("Final message: {}", message);
}

// Takes an immutable reference to a String
fn print_name(name: &String) {
    println!("Hello, {}!", name);
}

// Takes a mutable reference to a String and changes it
fn append_world(msg: &mut String) {
    msg.push_str(", world!");
}



fn part2(){
    println!("--- Bad Borrow Example ---");

    let mut text = String::from("Hello");

    // We create a mutable reference
    let text_ref = &mut text;

    // ERROR: Cannot use `text` while it's mutably borrowed
    //println!("Original text: {}", text);

    text_ref.push_str(", world!");
    println!("Modified via ref: {}", text_ref);

    // Now it's safe to use `text` again
    // The mutable borrow (`text_ref`) is no longer used
    println!("Now we can print: {}", text);

    println!("\n--- Option Pattern Matching ---");

    let maybe_name = Some("Rustacean");

    match maybe_name {
        Some(name) => println!("Welcome, {}!", name),
        None => println!("No name provided."),
    }

    // Trying a None case
    let none_case: Option<&str> = None;

    match none_case {
        Some(name) => println!("This shouldn't print."),
        None => println!("Handled the None case gracefully."),
    }
}