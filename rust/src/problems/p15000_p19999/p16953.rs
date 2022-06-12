/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::collections::VecDeque;
fn bfs(a: usize, b: usize) -> isize {
    let mut deq = VecDeque::new();
    deq.push_back(a);
    let mut next = 0;
    let mut level = 1;
    let mut count = 1;
    while let Some(x) = deq.pop_front() {
        if x == b { return count }
        if let Some(y) = x.checked_shl(1) {
            if y <= b {
                deq.push_back(y);
                next += 1;
            }
        }
        if let Some(z) = x.checked_mul(10).and_then(|y| y.checked_add(1)) {
            if z <= b {
                deq.push_back(z);
                next += 1;
            }
        }
        level -= 1;
        if level == 0 {
            level = next;
            next = 0;
            count += 1;
        }
    }
    -1
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [a, b] = [read(), read()];
    let ans = bfs(a, b);
    println!("{}", ans);
}