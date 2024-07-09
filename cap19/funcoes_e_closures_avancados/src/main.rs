fn main() {
    ponteiro_de_funcao();
    funcao_no_lugar_de_closure();
    enum_no_lugar_de_closure();
}

fn ponteiro_de_funcao() {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }

    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");
}

fn funcao_no_lugar_de_closure() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
    dbg!(&list_of_strings);
}

fn enum_no_lugar_de_closure() {
    #[derive(Debug)]
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0..=10).map(Status::Value).collect();
    dbg!(&list_of_statuses);
}
