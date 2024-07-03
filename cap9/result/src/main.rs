use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    panic_quando_retornar_result_error();
    panic_quando_retornar_result_error_com_unwrap();
    panic_quando_retornar_result_error_com_expect();
    result_com_pattern_matching();
    result_com_closures_e_unwrap_or_else();
    propagando_erros();
}

fn panic_quando_retornar_result_error() {
    // let greeting_file_result = File::open("hello.txt");
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {error:?}"),
    // };
}

fn panic_quando_retornar_result_error_com_unwrap() {
    // let _greeting_file_result = File::open("hello.txt").unwrap();
}

fn panic_quando_retornar_result_error_com_expect() {
    // let _greeting_file_result =
    //     File::open("hello.txt").expect("hello.txt should be included in this project");
}

fn result_com_pattern_matching() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    dbg!(&greeting_file);
}

fn result_com_closures_e_unwrap_or_else() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file: {error:?}"))
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn propagando_erros() {
    read_username_from_file();
    read_username_from_file_short_handler();
    read_username_from_file_very_short_handler();
    read_username_from_file_one_line();

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = match File::open("hello.txt") {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut username = String::new();
        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    fn read_username_from_file_short_handler() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file_very_short_handler() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file_one_line() -> Result<String, io::Error> {
        fs::read_to_string("hello.text")
    }
}
