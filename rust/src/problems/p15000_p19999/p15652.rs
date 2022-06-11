/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::fmt::Write;
fn f(n: u8, m: u8, last: u8, take: String, o: &mut String) {
    if m == 0 { return writeln!(o, "{}", take.trim_start()).unwrap() }
    for node in last..=n {
        f(n, m - 1, node, format!("{} {}", take, node), o);
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<u8>().unwrap();
    let [n, m] = [read(), read()];
    let mut o = String::new();
    f(n, m, 1, String::with_capacity(m as usize), &mut o);
    println!("{}", o);
}