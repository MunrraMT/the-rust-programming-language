fn main() {
    init();
    println!("---------");
    string_slices();
}

fn init() {
    let s = String::from("hello world");

    let word = first_word(&s);
    println!("the first word is: {word}");

    println!("----init----");

    let word = first_word_with_slice(&s);
    println!("the first word is: {word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn string_slices() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello} {world}");

    let slice = &s[..2];

    println!("{slice}");

    let slice = &s[2..];

    println!("{slice}");
}

fn first_word_with_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
