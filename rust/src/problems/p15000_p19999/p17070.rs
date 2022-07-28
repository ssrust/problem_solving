/// Author: juyoung35

use std::io::{stdin, prelude::*};
const EMPTY: u8 = 0;
const WALL : u8 = 1;
const HORIZONTAL: usize = 0;
const DIAGONAL  : usize = 1;
const VERTICAL  : usize = 2;
fn f(n: usize, house: &Vec<Vec<u8>>, dp: &mut Vec<Vec<Vec<usize>>>, y: usize, x: usize, s: usize) -> usize {
    let (ny, nx) = match s {
        HORIZONTAL => (y    , x + 1),
        DIAGONAL   => (y + 1, x + 1),
        VERTICAL   => (y + 1, x    ),
        _ => unreachable!(),
    };
    if ny == n - 1 && nx == n - 1 { return 1 }
    dp[y][x][s] = 0;
    if s != HORIZONTAL {
        if ny + 1 < n {
            if house[ny + 1][nx] == EMPTY {
                dp[y][x][s] += f(n, house, dp, ny, nx, VERTICAL);
            }
        }
    }
    if s != VERTICAL {
        if nx + 1 < n {
            if house[ny][nx + 1] == EMPTY {
                dp[y][x][s] += f(n, house, dp, ny, nx, HORIZONTAL);
            }
        }
    }
    if ny + 1 < n && nx + 1 < n {
        if house[ny + 1][nx] == EMPTY && house[ny][nx + 1] == EMPTY && house[ny + 1][nx + 1] == EMPTY {
            dp[y][x][s] += f(n, house, dp, ny, nx, DIAGONAL);
        }
    }
    dp[y][x][s]
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<u8>().unwrap();
    let n = read() as usize;
    let mut house = vec![vec![EMPTY; n]; n];
    for y in 0..n {
        for x in 0..n {
            house[y][x] = read();
        }
    }
    let mut dp = vec![vec![vec![0; 3]; n]; n];
    let ans = f(n, &house, &mut dp, 0, 0, HORIZONTAL);
    println!("{}", ans);
}