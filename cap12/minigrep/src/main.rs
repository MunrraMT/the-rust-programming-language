use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();

    dbg!(end - start);
}
