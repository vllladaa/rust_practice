// fn main() {
//     println!("Success!");
// }
//
// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };
//
//     // Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }
//
// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//
// }

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn()
}

// IMPLEMENT this function
// DON'T change any code else
fn never_return_fn() -> ! {
    unimplemented!()
}