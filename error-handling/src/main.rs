use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let handler = File::open("test.txt");

    let file = match handler {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("test.txt") {
                Ok(f) => f,
                Err(e) => panic!("Unable to create file: {:?}", e),
            },
            other => panic!("Unable to create file:  {:?}", other),
        },
    };

    println!("{:#?}", file);

    // Alternative concise approach
    let file = File::open("hello.text").unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|err| panic!("Unable to create file: {:?}", err))
        } else {
            panic!("Unable to open file: {:?}", err)
        }
    });

    println!("{:#?}", file);

    let file = File::open("unwrap.txt").unwrap(); // unwrap and default panic
    println!("{:#?}", file);

    let file = File::open("expect").expect("Unable to open file"); // panic with custom message
    println!("{:#?}", file);
    println!("{:#?}", read_username_from_file());
    println!("{:#?}", read_username_from_file_short());
    println!("{:#?}", read_username_from_file_shorter());
    println!("{:#?}", read_username_from_file_shortest());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("file.txt");

    let mut file = match file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => return Err(err),
    }
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut file = File::open("hello.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
