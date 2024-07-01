fn main() {
    tipo_struct();
}

fn tipo_struct() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user_1 = User {
        active: true,
        username: String::from("some_username"),
        email: String::from("some_email@example.com"),
        sign_in_count: 1,
    };

    user_1.email = String::from("another_email@exemple.com");

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    fn build_user_com_field_init_shorthand(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

fn instanciando_a_partir_de_outras_structs() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user_1 = User {
        active: true,
        username: String::from("some_username"),
        email: String::from("some_email@example.com"),
        sign_in_count: 1,
    };

    let user_2 = User {
        active: user_1.active,
        username: user_1.username,
        email: String::from("another@example.com"),
        sign_in_count: user_1.sign_in_count,
    };
}

fn instanciando_de_outras_structs_com_sintaxe_struct_update() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user_1 = User {
        active: true,
        username: String::from("some_username"),
        email: String::from("some_email@example.com"),
        sign_in_count: 1,
    };

    let user_2 = User {
        email: String::from("another_email_2@example.com"),
        ..user_1
    };
}

fn criando_tuple_structs() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
}

fn criando_unity_like_struct() {
    struct AlwaysEqual;

    fn main() {
        let subject = AlwaysEqual;
    }
}
