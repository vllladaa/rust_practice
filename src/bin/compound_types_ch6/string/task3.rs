//
// // Fill the blank
// fn main() {
//     let mut s = __;
//     s.push_str("hello, world");
//     s.push('!');
//
//     assert_eq!(s, "hello, world!");
//
//     println!("Success!");
// }

fn main() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");
}