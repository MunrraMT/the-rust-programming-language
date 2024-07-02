fn main() {
    criando_vetor();
    println!("-------");
    atualizando_vetor();
    println!("-------");
    lendo_vetores();
    println!("-------");
    iterando_vetor();
    println!("-------");
    vetor_de_enum();
}

fn criando_vetor() {
    let iniciando_vetor_vazio: Vec<i32> = Vec::new();
    let iniciando_vetor_com_itens = vec![1, 2, 3];

    dbg!(iniciando_vetor_vazio);
    dbg!(iniciando_vetor_com_itens);
}

fn atualizando_vetor() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    dbg!(v);
}

fn lendo_vetores() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {third}");

    let third = &v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn iterando_vetor() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v_mut = vec![100, 32, 57];
    for i in &mut v_mut {
        *i += 50;
    }

    dbg!(v_mut);
}

fn vetor_de_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    dbg!(row);
}
