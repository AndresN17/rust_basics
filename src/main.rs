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
    //Positional arguments can be used in println!
    println!(
        "{0} is a servant of the class {1}.",
        "Artoria Pendragon", "Saber"
    );
    //Can be named arguments too
    println!(
        "{servant} is a servant of the class {class}",
        servant = "Artoria Pendragon",
        class = "Saber"
    );

    //Special formatting can be use with a `:`
    println!(
        "{} of {:b} people know how to react in a specific situation.",
        1, 2
    );
    //Right align a text with a specific width
    println!("{number:>width$}", number = 1, width = 6);

    //You can't print structures in println! you have to manage this in a different way
    #[allow(dead_code)]
    struct Structure(i32);

    //This won't work
    //println!("This structure {}", Structure(32));

    //Primitive types on Rust
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let a_intefer = 5i32;

    let default_float = 1.0;
    let default_integer = 7;

    let mut inferred_type = 90;
    inferred_type = 281910i64;

    let mut mutable = 12;
    mutable = 35;

    //The type is importan!
    println!("1 + 2 = {}", 1u32+2);

    println!("1 - 2 = {}", 1i32-2);

    //Short-circuiting booleans
    println!("true AND false = {}", true && false);
    println!("true OR false = {}", true || false);
    println!("NOT true = {}",!true);

    //Inclusive ranges!
    for i in 1..3 {
        println!("i: {}", i);
    }
    //New inclusive ranges
    for i in 1..=3 {
        println!("i: {}", i);
    }
}
