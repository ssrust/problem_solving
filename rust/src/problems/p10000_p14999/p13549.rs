/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::collections::VecDeque;
const MAX: usize = 2 << 17;
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, k] = [read(), read()];
    let mut deq = VecDeque::new();
    let mut v = vec![false; MAX + 1];
    deq.push_back((0, n));
    v[n] = true;
    while let Some((d, i)) = deq.pop_front() {
        if i == k { return println!("{}", d) }
        if i <= MAX >> 1 && i < k << 1 {
            if !v[i << 1] {
                deq.push_front((d, i << 1));
                v[i << 1] = true;
            }
        }
        if i > 0 {
            if !v[i - 1] {
                deq.push_back((d + 1, i - 1));
                v[i - 1] = true;
            }
        }
        if i < MAX {
            if !v[i + 1] {
                deq.push_back((d + 1, i + 1));
                v[i + 1] = true;
            }
        }
    }
}