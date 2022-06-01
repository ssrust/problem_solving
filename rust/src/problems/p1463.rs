use std::io::{stdin, prelude::*};

fn main() {
    let n: usize = stdin().lock().lines().take(1).next().unwrap().unwrap().parse().unwrap();
    let mut v = vec![0; n + 1];
    for i in 2..=n {
        v[i] = v[i - 1] + 1;
        if i % 2 == 0 { v[i] = v[i].min(v[i / 2] + 1) }
        if i % 3 == 0 { v[i] = v[i].min(v[i / 3] + 1) }
    }
    println!("{}", v[n]);
}