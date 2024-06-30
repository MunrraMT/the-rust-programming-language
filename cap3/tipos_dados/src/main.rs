fn main() {
    tipo_flutuante();
    operacoes_numericas();
    boleano();
    tipo_caractere();
    tipo_composto_tupla();
    tipo_composto_array();
}

fn tipo_flutuante() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

fn operacoes_numericas() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn boleano() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

fn tipo_caractere() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn tipo_composto_tupla() {
    let exemplo_tupla: (i32, f64, u8, &str) = (500, 6.4, 1, "foi?");

    let (x, y, z, text) = exemplo_tupla;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn tipo_composto_array() {
    let exemplo_array = [1, 2, 3, 4, 5];

    let first = exemplo_array[0];
    let second = exemplo_array[1];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
