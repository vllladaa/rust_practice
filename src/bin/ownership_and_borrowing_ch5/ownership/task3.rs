//
// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }
//
// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("Hello world");
//     // Convert String to Vec
//     let _s = s.into_bytes();
//     s
// }

fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    let _s = s.as_bytes();
    s
}