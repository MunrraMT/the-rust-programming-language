use std::fmt::{Debug, Display};

fn main() {
    definindo_trait();
    println!("-----------");
    implementando_trait();
    println!("-----------");
    implementacao_padrao();
    println!("-----------");
    implementacao_padrao_2();
    println!("-----------");
    trait_como_parametro_de_funcao();
    println!("-----------");
    trait_bound();
    println!("-----------");
    trait_bound_com_where_clauses();
    println!("-----------");
    retornando_impl_trait();
}

fn definindo_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }
}

fn implementando_trait() {
    pub trait Summary {
        fn summarize(&self) -> String;
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    fn use_trait() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    use_trait();
}

fn implementacao_padrao() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

fn implementacao_padrao_2() {
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

fn trait_como_parametro_de_funcao() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // Só funciona se o tipo tiver implementado o Summary
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
}

fn trait_bound() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // item1 e item2 podem ter implementações diferentes do Summary
    pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

    // os dois itens precisam ter a mesma implementação do Summary
    pub fn notify_2<T: Summary>(item1: &T, item2: &T) {}

    // o item precisa ser de um tipo que tenha implementado Summary e Display
    pub fn notify_3(item: &(impl Summary + Display)) {}

    // item1 e item2 devem ser tipos que tenham a implementação de Summary e de Display
    pub fn notify_4<T: Summary + Display>(item1: &T, item2: &T) {}
}

fn trait_bound_com_where_clauses() {
    fn function_without_where<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        10
    }

    fn function_with_where<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        10
    }
}

fn retornando_impl_trait() {
    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {}

    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

fn trait_bound_com_condicionais() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
}
