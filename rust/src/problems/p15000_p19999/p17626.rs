/// Author: juyoung35

use std::io::{stdin, prelude::*};
fn brute_force(squares: &Vec<usize>, len: usize, n: usize, sum: usize, nth: usize) -> bool {
    if nth == 0 {
        if sum == n { return true }
        return false;
    }
    for i in 0..len {
        let sum = sum + squares[i];
        if sum > n { continue }
        if brute_force(squares, len, n, sum, nth - 1) { return true }
    }
    false
}
fn main() {
    let squares = (1..=50_000f64.sqrt() as usize).map(|x| x * x).collect::<Vec<usize>>();
    let len = squares.len();
    let n: usize = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    for nth in 1..=4 {
        if brute_force(&squares, len, n, 0, nth) {
            return println!("{}", nth);
        }
    }
}