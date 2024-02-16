use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

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


    // Channel
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
            thread::sleep(Duration::from_millis(10));
        }
    });

    for val in rx {
        println!("Received :  {}", val);
    }

    // Mutex
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut m = counter.lock().unwrap();

            *m += 1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Shared mutated value: {:?}", *counter.lock().unwrap());

}
