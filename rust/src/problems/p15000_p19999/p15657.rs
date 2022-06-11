/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::fmt::Write;
fn f(n: usize, m: usize, left: &[usize], take: String, o: &mut String) {
    if m == 0 { return writeln!(o, "{}", take.trim_start()).unwrap() }
    for i in 0..left.len() {
        f(n, m - 1, &left[i..], format!("{} {}", take, left[i]), o);
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, m] = [read(), read()];
    let mut left = Vec::with_capacity(n);
    (0..n).for_each(|_| left.push(read()));
    left.sort_unstable();
    let mut o = String::new();
    f(n, m, &mut left, String::new(), &mut o);
    println!("{}", o);
}