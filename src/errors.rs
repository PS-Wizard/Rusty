// use std::{fs::File, io::ErrorKind, panic};

use std::{fs::File, panic};

// use std::panic;
//
// fn a() {
//     b();
// }
//
// fn b() {
//     c(69);
// }
//
// fn c(num: i32) {
//     panic!("69 lol {num}");
// }
//
pub fn errors() {
    // a();
    // let filehandle = File::open("./sdfsf.rs");
    // let f = match filehandle {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         std::io::ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error creating file"),
    //         },
    //         _ => panic!("Problem creating a file"),
    //     },
    // };
    //

    // let filehandle = File::open("./sdfsf.rs").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| panic!("problem creating file"))
    //     } else {
    //         panic!("Problem opening file");
    //     }
    // });

    // let f = File::open("./main.rs")    
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => panic!("{err}"),
    // }

    // let f = File::open("./main.rs").unwrap() // panics if fails
    // let f = File::open("./main.rs").expect("Error openign file");
}
