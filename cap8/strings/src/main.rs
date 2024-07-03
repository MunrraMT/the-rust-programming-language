fn main() {
    nova_string();
    println!("------------");
    atualizando_string();
    println!("------------");
    concatenando_strings();
}

fn nova_string() {
    let data = "initial contents";
    let s = data.to_string();
    dbg!(s);

    let s = "initial contents".to_string();
    dbg!(s);

    let s = String::from("initial contents");
    dbg!(s);
}

fn atualizando_string() {
    let mut s = String::from("foo");
    s.push_str("bar");
    dbg!(s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');
    dbg!(s);
}

fn concatenando_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    dbg!(s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s_total = s1 + "-" + &s2 + "-" + &s3;
    dbg!(s_total);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s_total = format!("{s1}-{s2}-{s3}");
    dbg!(s_total);
}
