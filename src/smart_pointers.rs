use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn pointy_pointy_lol() {
    let a = Rc::new(List::Cons(5, Rc::new(List::Cons(10, Rc::new(List::Nil)))));
    let b = List::Cons(3, Rc::clone(&a));
    let c = List::Cons(3, Rc::clone(&a));

    // // // // let b = Box::new(5);
    // // // // println!("{b}");
    // // // let list = List::Cons(
    // // //     1,
    // // //     Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    // // // );
    // //
    // // let x = 5;
    // // let y = MyBox::new(x);
    // // assert_eq!(5, x);
    // // assert_eq!(5, *y);
    // //
    // // let m = MyBox::new(String::from("Rust"));
    // // hello(&m)
    //
    // let c = CustomSmartPointer {
    //     data: String::from("Hello"),
    // };
    //
    // let d = CustomSmartPointer {
    //     data: String::from("Another Hello"),
    // };
    //
    // println!("Created");
}

//
// use std::ops::Deref;
//
// struct MyBox<T>(T);
//
// impl<T> MyBox<T> {
//     fn new(x: T) -> Self {
//         MyBox(x)
//     }
// }
//
// impl<T> Deref for MyBox<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping pointer with data {}", self.data);
    }
}

// fn hello(name: &str) {
//     println!("hello {name}");
// }
