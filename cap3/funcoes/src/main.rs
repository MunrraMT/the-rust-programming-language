fn main() {
    another_function(10);
    print_labeled_measurement(5, 'h');
    five();
    ten();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn ten() -> u32 {
    return 10;
}
