/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
use std::collections::HashMap;
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut next = || it.next().unwrap();

    let [n, m] = [0; 2].map(|_| next().parse::<usize>().unwrap());
    let mut map = HashMap::with_capacity(n);
    for _ in 0..n {
        map.insert(next(), next());
    }

    let mut o = BufWriter::new(io::stdout());
    for _ in 0..m {
        writeln!(o, "{}", map[next()]).unwrap();
    }
}
