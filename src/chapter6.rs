// enum Message {
//     Quit,
//     Move { x: u32, y: u32 },
//     Write(String),
//     ChangeColor(u32, u32, u32),
// }

// impl Message {
//     pub fn somefunc() {
//         println!("SOme func")
//     }
// }
//
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }
pub fn chapter6() {
    // // let x = 5;
    // // let y = Some(5);
    // //
    // // let some = x + y.unwrap_or(0);
    // // value_in_cents(Coin::Penny);
    // //
    // let five = Some(5);
    // let six = plus_one(five);
    // println!("{}", six.unwrap_or(-1));
    // let none = plus_one(None);
    // println!("{}", none.unwrap_or(-1));

    let some_value = Some(69);

    if let Some(68) = some_value {
        println!("Got: ");
    }
}

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky Penny");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
