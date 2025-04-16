fn main() {
    //Variables
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of X is {x}");
    }

    println!("The value of X is {x}.");

    let _guess: u32 = "42".parse().expect("Not a number!");

    //floating pints
    //let x = 2.0; // f64

    //let y: f32 = 3.0; // f32
    
    func();
}

fn func(){ 
    println!("Hello there");
}
