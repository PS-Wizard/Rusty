pub fn chapter3() {
    // Mutability:
    // Every value in rust is immutable by default:

    // let x = 69;
    // x = 52; <- Can't Do This

    //let mut x = 69;
    //x = 52; <- Can Do This

    // Declaring A Const
    // const SOMETHING: u32 = 69_420;
    // Const can't be mutated, and needs to be known at compile time.

    // Scalar Data Types:
    // Integers
    // Floating numbers
    // Booleans
    // Characters

    // Compound Types:
    let tuple = ("Something", 100_000);
    let (_, number) = tuple;
    let text = tuple.0;
    println!("{text}, {number}");

    loop {
        println!("hi");
        break;
    }

    for number in 1..5 {
        println!("{number}");
    }
}
