/// Author: juyoung35

use std::io::{stdin, prelude::*};
fn f(a: usize, b: usize, c: usize) -> usize {
    if b == 0 { return 1 }
    if b == 1 { return a % c }
    let mut res = f(a, b % 2, c);
    res *= f(a, b >> 1, c).pow(2) % c;
    res % c
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [a, b, c] = [0; 3].map(|_| read());
    let ans = f(a, b, c);
    println!("{}", ans);
}