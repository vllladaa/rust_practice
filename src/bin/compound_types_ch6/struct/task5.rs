//
// // Fill the blank
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     println!("Success!");
// }
//
// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         __
//     }
// }

struct Person {
    name: String,
    age: u8,
}
fn main() {}

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name
    }
}