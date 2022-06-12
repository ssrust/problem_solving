/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let t = read();
    let mut o = BufWriter::new(io::stdout());
    for _ in 0..t {
        let n = read();
        let mut stickers = vec![vec![0; n]; 2];
        for i in 0..2 {
            for j in 0..n {
                stickers[i][j] = read();
            }
        }
        let mut dp = vec![vec![0; n]; 2];
        for i in 0..2 {
            dp[i][0] = stickers[i][0];
        }
        for j in 1..n {
            for i in 0..2 {
                dp[i][j] = usize::max(dp[i ^ 1][j.checked_sub(2).unwrap_or(j - 1)], dp[i ^ 1][j - 1]) + stickers[i][j];
            }
        }
        let ans = usize::max(dp[0][n - 1], dp[1][n - 1]);
        writeln!(o, "{}", ans).unwrap();
    }
}