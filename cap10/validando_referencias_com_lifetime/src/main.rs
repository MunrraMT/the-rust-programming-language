use std::fmt::Display;

fn main() {
    anotacoes_de_tempo_de_vida_em_assinaturas_de_funcao();
    println!("-------------");
    funcao_com_tipo_generico_com_trait_bound_e_com_lifetime();
}

fn anotacoes_de_tempo_de_vida_em_assinaturas_de_funcao() {
    fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // Usando a anotação de lifetime na função, o compilador força as variáveis usadas como argumentos na função a manterem seu ownership, podendo ser feito apenas borrowing;

    let x = String::from("abc");
    let y = String::from("abcd");

    let z = &y;

    dbg!(longest(&x, &y), z);
}

fn funcao_com_tipo_generico_com_trait_bound_e_com_lifetime() {
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
