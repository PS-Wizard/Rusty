// use std::{thread, time::Duration};

// use std::sync::{self, mpsc};

// pub fn brrr() {
//     // // let handle = thread::spawn(|| {
//     // //     for i in 1..10 {
//     // //         println!("hi {i}");
//     // //         thread::sleep(Duration::from_millis(1));
//     // //     }
//     // // });
//     // // handle.join().unwrap();
//     // // for i in 1..5 {
//     // //     println!("hi from non thread{i}");
//     // //     thread::sleep(Duration::from_millis(1));
//     // // }
//     //
//     // let v = vec![1, 2, 3];
//     // let handle = thread::spawn(move || {
//     //     println!("Here is a vector: {:?}", v);
//     // });
//     //
//     // handle.join().unwrap();
//     let (tx, rx) = mpsc::channel();
//     let tx2 = tx.clone();
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("concurrent"),
//         ];
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi 1"),
//             String::from("from 1"),
//         ];
//         for val in vals {
//             tx2.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received in rx {
//         println!("Got: {received}");
//     }
// }
//
pub fn brrr() {
    let m = sync::Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m);
}
