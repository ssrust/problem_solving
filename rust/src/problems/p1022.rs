use std::io::{self, prelude::*, BufWriter};
fn get(x: isize, y: isize) -> isize {
    let layer = x.abs().max(y.abs());
    let quarter = layer << 1;
    let cells = (quarter - 1).pow(2);
    if x == layer && y == layer { ((layer + 1 << 1) - 1).pow(2) }
    else if x ==  layer { cells + (layer - y) }
    else if y == -layer { cells + quarter + (layer - x) }
    else if x == -layer { cells + 3 * quarter + (y - layer) }
    else if y ==  layer { cells + 4 * quarter + (x - layer) }
    else { unreachable!() }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<isize>().unwrap();
    let [r1, c1, r2, c2] = [0; 4].map(|_| read());
    let max = isize::max(isize::max(get(c1, r1), get(c1, r2)), isize::max(get(c2, r1), get(c2, r2)));
    let len = format!("{}", max).len();
    let mut o = BufWriter::new(io::stdout());
    for row in r1..=r2 {
        for col in c1..=c2 {
            write!(o, "{:len$} ", get(col, row), len=len).unwrap();
        }
        writeln!(o).unwrap();
    }
}