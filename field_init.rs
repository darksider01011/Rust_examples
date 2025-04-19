struct Person {
        name: String,
        age: u8,
 }

fn create_person(age: u8) -> Person {
    Person {
        name: String::from("Alice"),
        age,
    }
}


fn main() {
    let person = create_person(50);
    println!("{} is {} years old", person.name, person.age);
}
