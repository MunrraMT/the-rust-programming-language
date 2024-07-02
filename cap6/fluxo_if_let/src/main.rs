fn main() {
    fluxo_match();
    println!("----------");
    fluxo_if_let();
}

fn fluxo_match() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _other => (),
    }
}

fn fluxo_if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
