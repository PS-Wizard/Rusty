#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

trait Calculator {
    fn area(&self) -> u32;
}

impl Calculator for Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
pub fn chapter5() {
    let rect = Rectangle {
        width: 5,
        height: 5,
    };
    println!("{:#?}", rect);
    println!("{}", rect.area());
    // let mut user1 = User {
    //     username: String::from("Hello"),
    //     email: String::from("someemail@gmail.com"),
    //     sign_in_count: 69,
    //     active: false,
    // };
    //
    // user1.username = String::from("someotherusername");
    // let user2 = build_user("Someemail@gmail.com", "some user name someotherusername");
    //
    // let user3 = User {
    //     email: "user3@gmail.com".to_owned(),
    //     ..user2
    // };
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: false,
//         sign_in_count: 69,
//     }
// }
