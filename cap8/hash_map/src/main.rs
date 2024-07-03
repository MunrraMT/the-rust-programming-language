use std::collections::HashMap;

fn main() {
    novo_hash_map();
    println!("-----------");
    lendo_hash_map();
    println!("-----------");
    hash_map_ownership();
    println!("-----------");
    update_hash_map();
}

fn novo_hash_map() -> HashMap<String, u32> {
    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    dbg!(&scores);

    scores
}

fn lendo_hash_map() {
    let scores = novo_hash_map();
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    dbg!(&score);

    for (key, value) in scores {
        println!("{key}: {value}");
    }
}

fn hash_map_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();

    map.insert(field_name, field_value);
    dbg!(map);
}

fn update_hash_map() {
    substituindo_valor();
    adicionando_valor_caso_chave_nao_exista();
    usando_valor_antigo_para_fazer_novo();

    fn substituindo_valor() {
        let mut scores = HashMap::new();
        scores.insert(String::from("BLue"), 10);
        scores.insert(String::from("BLue"), 25);
        dbg!(&scores);
    }

    fn adicionando_valor_caso_chave_nao_exista() {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        dbg!(&scores);
    }

    fn usando_valor_antigo_para_fazer_novo() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        dbg!(map);
    }
}
