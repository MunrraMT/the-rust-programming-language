fn main() {
    referencia();
    println!("--------");
    referencia_mutavel();
    println!("--------");
    referencia_imutavel_com_mutavel();
}

fn referencia() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}

fn referencia_mutavel() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{s}");

    fn change(some_string: &mut String) {
        some_string.push_str(", world!");
    }
}

fn referencia_imutavel_com_mutavel() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");
}
