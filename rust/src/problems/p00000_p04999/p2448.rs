/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
fn f(n: usize, lines: &mut Vec<String>) {
    if n == 3 {
        lines[0] = "*".to_string();
        lines[1] = "* *".to_string();
        lines[2] = "*".repeat(5);
        return
    }
    let m = n >> 1;
    f(m, lines);
    for i in m..n {
        lines[i] = format!("{0}{1}{0}", lines[i - m], " ".repeat((n - i << 1) - 1));
    }
}
fn main() {
    let n: usize = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let mut lines = vec![String::with_capacity((n << 1) - 1); n];
    f(n, &mut lines);
    let mut o = BufWriter::new(io::stdout());
    for i in 0..n {
        writeln!(o, "{:^width$}", lines[i], width=(n << 1) - 1).unwrap();
    }
}