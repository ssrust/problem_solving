/// Author: juyoung35

use std::io::{stdin, prelude::*};
fn permutation(n: u128, r: u128, inv: &mut Vec<u128>) -> u128 {
    if n == r { return 1 }
    let mut res = n * permutation(n - 1, r, inv);
    let len = inv.len();
    for i in (0..len).rev() {
        if res % inv[i] == 0 {
            res /= inv[i];
            inv.remove(i);
        }
    }
    res
}
fn combination(n: u128, m: u128) -> u128 {
    let r = u128::min(n - m, m);
    let mut inv = (1..=r).into_iter().collect();
    permutation(n, n - r, &mut inv)
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<u128>().unwrap();
    let [n, m] = [0; 2].map(|_| read());
    let ans = combination(n, m);
    println!("{}", ans);
}