#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    favorite_color: Color,
}

// enumerated type
#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name = {}, age = {} has {} children",
            self.name, self.age, self.children
        )
    }
}

fn handler() {
    let _p = Person {
        name: "matt".to_string(),
        age: 35,
        children: 4,
        favorite_color: Color::Green,
    };

    let c = Color::Red("hello".to_string());

    match c {
        Color::Red(s) => println!("It's red! {s}"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
    }

    println!("Hello, people, from {:?}", _p);
}
