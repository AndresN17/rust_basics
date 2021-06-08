//Basic features of RUST Language
fn main() {
    //Declaration of variables
    //This variable is inmutable
    let nombre = "Andres Noboa";
    //Printing a variable with println! function
    println!("My name is {} and this is my first work in Rust!", nombre);
    //For make a variable mutable you have to add mut
    let mut mut_number = 1;
    println!("This is a mutable variable {}", mut_number);
    mut_number += 1;
    println!("This is a mutable variable changing {}", mut_number);

    //Inclusive ranges!
    for i in 1..3 {
        println!("i: {}", i);
    }
    //New inclusive ranges
    for i in 1..=3 {
        println!("i: {}", i);
    }
}
