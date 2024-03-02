use async_std::{fs::File, io, prelude::*, task};


async fn read_file(path: &str) -> io::Result<String> {
    let mut file  =  File::open(path).await?;
    let mut content  = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}

fn main() {
    let _task = task::block_on(async {
        let result = read_file("file.txt").await;

        match result {
            Ok(data) => println!("{}", data),
            Err(err) => println!("Error: {}", err) 
        }
    });

    // task::spawn( async {}): spawn async task and return join handle
    // task::block_on( async {}): spawn async task in a thread and join immediately, 
}
