/// Author: juyoung35
/// this algorithm takes 116ms
/// it can be solved less than 4ms without loop
/// answer x: w - w.abs_diff((p + t) % (w << 1))
/// answer y: h - h.abs_diff((q + t) % (h << 1))

use std::io::{stdin, prelude::*};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<isize>().unwrap();
    let [w, h] = [read(), read()];
    let [p, q] = [read(), read()];
    let t = read();
    let [mut x, mut y] = [p + t, q + t];
    loop {
        if x < 0 { x *= -1 }
        if x > w { x = -x + (w << 1) }
        if y < 0 { y *= -1 }
        if y > h { y = -y + (h << 1) }
        if (0 <= x && x <= w) && (0 <= y && y <= h) { break }
    }
    println!("{} {}", x, y);
}