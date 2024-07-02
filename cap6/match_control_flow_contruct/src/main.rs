fn main() {
    match_por_enum();
    println!("------------");
    match_por_padrao();
    println!("------------");
    match_com_option();
    println!("------------");
    match_com_branch_padrao();
    println!("------------");
    match_com_branch_padrao_nao_usada();
    println!("------------");
    match_com_branch_padrao_vazio();
}

fn match_por_enum() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter);

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn match_por_padrao() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alabama));
    value_in_cents(Coin::Quarter(UsState::Alaska));

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }
}

fn match_com_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(six);
    dbg!(none);
}

fn match_com_branch_padrao() {
    let dice_roll: u8 = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(_num_spaces: u8) {}
}

fn match_com_branch_padrao_nao_usada() {
    let dice_roll: u8 = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _other => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

fn match_com_branch_padrao_vazio() {
    let dice_roll: u8 = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _other => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
