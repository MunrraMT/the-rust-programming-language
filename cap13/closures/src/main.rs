use closures::{Inventory, Rectangle, ShirtColor};

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref_1 = Some(ShirtColor::Red);
    let giveaway_1 = store.giveaway(user_pref_1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_1, giveaway_1
    );

    let user_pref_2 = None;
    let giveaway_2 = store.giveaway(user_pref_2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref_2, giveaway_2
    );

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });

    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
