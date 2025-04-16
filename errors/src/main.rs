fn main() {
    println!("{}", hello(String::from("World")));
}

fn hello(name: String) -> String{
    return "Hello ".to_owned() + &name;
}
