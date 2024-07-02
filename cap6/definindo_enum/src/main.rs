fn main() {
    enums();
    option_enums();
}

fn enums() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {}
    }

    fn main() {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));

        let m = Message::Write(String::from("hello"));

        m.call();
    }

    main();
}

enum Option<T> {
    None,
    Some(T),
}

fn option_enums() {
    let some_number = Option::Some(5);
    let some_char = Option::Some('e');
    let absent_number: Option<i32> = Option::None;
}
