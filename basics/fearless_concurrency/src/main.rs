// use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
// use std::time::Duration;

fn main() {
    // let conc = thread::spawn(|| {
    //     for i in 1..=10 {
    //         println!("hi number {i} from the spawned thread!");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // conc.join().unwrap();

    // for i in 1..10 {
    //     println!("hi number {i} from the main thread!");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || println!("Here's a vector: {v:?}"));

    // handle.join().unwrap();

    // let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone();
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];
    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("more"),
    //         String::from("message"),
    //         String::from("for"),
    //         String::from("you"),
    //     ];
    //     for val in vals {
    //         tx1.send(val).unwrap();
    //         // thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {received}");
    // }

    let a = Arc::new(Mutex::new(1));

    let counter = Arc::clone(&a);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });

    handle.join().unwrap();

    println!("Got: {}", *a.lock().unwrap());
}
