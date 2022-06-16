/// Author: juyoung35

use std::io::{stdin, prelude::*};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, k] = [read(), read()];
    let mut dp = vec![0; k + 1];
    for _ in 0..n {
        let [w, v] = [read(), read()];
        for i in (w..=k).rev() {
            dp[i] = usize::max(v + dp[i - w], dp[i])
        }
    }
    println!("{}", dp[k]);
}