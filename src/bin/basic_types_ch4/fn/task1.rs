// fn main() {
//     // Don't modify the following two lines!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);
//
//     assert_eq!(s, 3);
//
//     println!("Success!");
// }
//
// fn sum(x, y: i32) {
//     x + y;
// }

fn main() {
    // don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}