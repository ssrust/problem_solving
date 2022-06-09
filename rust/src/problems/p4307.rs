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
        let [l, n] = [read(), read()];
        let mut fastest = 0;
        let mut slowest = 0;
        for _ in 0..n {
            let x = read();
            let fast = usize::min(x, x.abs_diff(l));
            if fast > fastest { fastest = fast }
            let slow = usize::max(x, x.abs_diff(l));
            if slow > slowest { slowest = slow }
        }
        writeln!(o, "{} {}", fastest, slowest).unwrap();
    }
}