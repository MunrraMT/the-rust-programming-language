fn main() {
    escopo_variavel();
    println!("---------");
    tipo_string();
    println!("---------");
    variaveis_dados_interagindo_com_move();
    println!("---------");
    variaveis_dados_interagindo_com_clone();
    println!("---------");
    dados_somente_de_pilha_copiar();
    println!("---------");
    propriedades_e_funcoes();
    println!("---------");
    valores_de_retorno_e_escopo();
    println!("---------");
    retorno_ownership_por_parametro();
}

fn escopo_variavel() {
    let _s = "hello";
}

fn tipo_string() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}

fn variaveis_dados_interagindo_com_move() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");
}

fn variaveis_dados_interagindo_com_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn dados_somente_de_pilha_copiar() {
    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");
}

fn propriedades_e_funcoes() {
    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }
}

fn valores_de_retorno_e_escopo() {
    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);

    fn gives_ownership() -> String {
        let some_string = String::from("yours");
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }
}

fn retorno_ownership_por_parametro() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("Theh length of '{s2}' is {len}");

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }
}
