use std::io::{stdin, prelude::*};
const RED  : usize = 0;
const GREEN: usize = 1;
const BLUE : usize = 2;
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let n = read();
    let mut dp = vec![vec![0; 3]; n + 1];
    for i in 0..n {
        dp[i + 1][RED  ] = usize::min(dp[i][GREEN], dp[i][BLUE ]) + read();
        dp[i + 1][GREEN] = usize::min(dp[i][BLUE ], dp[i][RED  ]) + read();
        dp[i + 1][BLUE ] = usize::min(dp[i][RED  ], dp[i][GREEN]) + read();
    }
    let ans = dp.last().unwrap().iter().min().unwrap();
    println!("{}", ans);
}