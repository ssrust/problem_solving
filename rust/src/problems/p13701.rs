/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
fn main() {
    let mut n = 0;
    let mut set = vec![0usize; 1 << 19];
    let mut o = BufWriter::new(io::stdout());
    io::stdin().bytes().map(|r| r.unwrap()).for_each(|a| {
        if a != b' ' && a != b'\n' { return n = 10 * n + (a - b'0') as usize }
        let b = n / (1 << 6);
        let c = n % (1 << 6);
        if set[b] & 1 << c != 1 << c {
            set[b] |= 1 << c;
            write!(o, "{} ", n).unwrap();
        }
        n = 0;
    });
}