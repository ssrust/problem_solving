/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::collections::VecDeque;
const EMPTY : u8 = 0b_0000;
const UNRIPE: u8 = 0b_0001;
const RIPE  : u8 = 0b_0010;
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut next = || it.next().unwrap();
    let [m, n, o, p, q, r, s, t, u, v, w] = [0; 11].map(|_| next().parse::<usize>().unwrap());
    let mut storage = vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![vec![EMPTY; m]; n]; o]; p]; q]; r]; s]; t]; u]; v]; w];
    let mut deq = VecDeque::new();
    let mut unripe = 0;
    let mut level = 0;    
    for xw in 0..w {
        for xv in 0..v {
            for xu in 0..u {
                for xt in 0..t {
                    for xs in 0..s {
                        for xr in 0..r {
                            for xq in 0..q {
                                for xp in 0..p {
                                    for xo in 0..o {
                                        for xn in 0..n {
                                            for xm in 0..m {
                                                storage[xw][xv][xu][xt][xs][xr][xq][xp][xo][xn][xm] = match next().chars().next().unwrap() {
                                                    '-' => continue,
                                                    '0' => {
                                                        unripe += 1;
                                                        UNRIPE
                                                    },
                                                    '1' => {
                                                        deq.push_back((xw, xv, xu, xt, xs, xr, xq, xp, xo, xn, xm));
                                                        level += 1;
                                                        RIPE
                                                    },
                                                    _ => unreachable!(),
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut next = 0;
    let mut count = -1;
    loop {
        let (mut xw, mut xv, mut xu, mut xt, mut xs, mut xr, mut xq, mut xp, mut xo, mut xn, mut xm) = if let Some(x) = deq.pop_front() { x } else { break };
        macro_rules! left {
            ($x: ident) => {
                if $x > 0 {
                    $x -= 1;
                    if storage[xw][xv][xu][xt][xs][xr][xq][xp][xo][xn][xm] == UNRIPE {
                        storage[xw][xv][xu][xt][xs][xr][xq][xp][xo][xn][xm] = RIPE;
                        deq.push_back((xw, xv, xu, xt, xs, xr, xq, xp, xo, xn, xm));
                        unripe -= 1;
                        next += 1;
                    }
                    $x += 1;
                }
            };
        }
        macro_rules! right {
            ($x: ident, $y: ident) => {
                if $x < $y - 1 {
                    $x += 1;
                    if storage[xw][xv][xu][xt][xs][xr][xq][xp][xo][xn][xm] == UNRIPE {
                        storage[xw][xv][xu][xt][xs][xr][xq][xp][xo][xn][xm] = RIPE;
                        deq.push_back((xw, xv, xu, xt, xs, xr, xq, xp, xo, xn, xm));
                        unripe -= 1;
                        next += 1;
                    }
                    $x -= 1;
                }
            };
        }
        left!(xw); right!(xw, w);
        left!(xv); right!(xv, v);
        left!(xu); right!(xu, u);
        left!(xt); right!(xt, t);
        left!(xs); right!(xs, s);
        left!(xr); right!(xr, r);
        left!(xq); right!(xq, q);
        left!(xp); right!(xp, p);
        left!(xo); right!(xo, o);
        left!(xn); right!(xn, n);
        left!(xm); right!(xm, m);
        level -= 1;
        if level == 0 {
            level = next;
            next = 0;
            count += 1;
        }
    }
    let ans = if unripe == 0 { count } else { -1 };
    println!("{}", ans);
}