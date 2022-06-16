/// Author: juoyung35

use std::io::{stdin, prelude::*};
const MOD: usize = 1_000_000_007;
type Matrix = [[usize; 2]; 2];
fn mul(m: Matrix, n: Matrix) -> Matrix {
    [
        [(m[0][0] * n[0][0] % MOD + m[0][1] * n[1][0] % MOD) % MOD, (m[0][0] * n[0][1] % MOD + m[0][1] * n[1][1] % MOD) % MOD],
        [(m[1][0] * n[0][0] % MOD + m[1][1] * n[1][0] % MOD) % MOD, (m[1][0] * n[0][1] % MOD + m[1][1] * n[1][1] % MOD) % MOD],
    ]
}
fn pow(m: Matrix) -> Matrix {
    mul(m, m)
}
fn f(n: usize) -> Matrix {
    if n == 1 {
        return [
            [1, 1],
            [1, 0],
        ]
    }
    if n & 1 != 0 {
        mul(pow(f(n >> 1)), f(1))
    } else {
        pow(f(n >> 1))
    }
}
fn main() {
    let n: usize = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let ans = f(n)[0][1];
    println!("{}", ans);
}