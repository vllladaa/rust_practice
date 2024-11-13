// Fix the error below with least amount of modification to the code
// fn main() {
//     let x: i32; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !
//
//     assert_eq!(x, 5);
//     println!("Success!");
// }

fn main() {
    let x: i32 = 5; // uninitialized but using, ERROR !
    let y: i32; // uninitialized but also unusing, only warning
    println!("{} is equal to 5", x);
}