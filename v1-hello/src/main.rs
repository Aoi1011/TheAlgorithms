#[derive(Debug, Clone)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    pub fn greet(&self) -> String {
        format!("Hi my name is {}", self.name)
    }

    pub fn age_up(&mut self, n: i32) {
        self.age += n
    }

    pub fn dropme(self) {}
}

fn main() {
    let mut p = Person::new("Mike".to_string(), 32);
    p.age_up(3);
    let s = p.greet();
    println!("{:?}", s);

    let a = get_age(&p);

    println!("person's age is {}", a);
    p.age_up(2);

    let s2 = p.greet();
    println!("really : {}", s2);

    // p.name = "Tom".to_string();
    // p.age_up(1);

    // let s2 = p.greet();

    // let a = get_age(&p);

    // println!("Person's age is {}", a);
}

pub fn get_age(s: &Person) -> &i32 {
    &s.age
}
