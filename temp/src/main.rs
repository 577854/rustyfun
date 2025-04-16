fn main() {

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}", s1); // This will cause a compile-time error
    println!("{}", s2); // This will work fine
    // The ownership of s1 has been moved to s2, so s1 is no longer valid
    // You can use s2 without any issues
    // Ownership and borrowing
    let s1 = String::from("hello");
    let s2 = &s1; // Borrowing s1, s2 is a reference to s1
    println!("{}", s1); // This will work fine
    println!("{}", s2); // This will work fine
}
