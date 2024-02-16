use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Spawn: {}", i);
            thread::sleep(Duration::from_millis(4));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![10, 12, 14];

    let handle = thread::spawn(move || {
        println!("{:#?}", v);
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap();
    });

    println!("Got: {:?}", rx.recv().unwrap());

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("my"),
            String::from("side"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for val in rx {
        println!("Received :  {}", val);
    }
}
