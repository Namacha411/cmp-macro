use cmp_macro::{le, lety};

fn main() {
    let x = 3;
    if le!(1, x, 4) {
        println!("1 <= x <= 4");
    }

    if lety!(i32; -2.0, -1, x, 4, 5.0) {
        println!("-2 <= -1 <= x <= 4 <= 5");
    }
}