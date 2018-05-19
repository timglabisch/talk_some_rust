struct Person {
    age: i32
}

impl Person {
    pub fn new(age: i32) -> Self {
        Person {
            age
        }
    }

    pub fn get_age(&self) -> i32 {
        self.age
    }
}

pub fn main() {
    let tim = Person::new(25);
    println!("tim is {} years old", tim.get_age());
}