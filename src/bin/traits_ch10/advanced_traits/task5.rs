// use std::fmt;
//
// // DEFINE a newtype `Pretty` here
//
//
// impl fmt::Display for Pretty {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "\"{}\"", self.0.clone() + ", world")
//     }
// }
//
// fn main() {
//     let w = Pretty("hello".to_string());
//     println!("w = {}", w);
// }

use std::fmt;

// DEFINE a newtype `Pretty`
struct Pretty(String);

impl fmt::Display for Pretty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\"{}\"", self.0.clone() + ", world")
    }
}

fn main() {
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}