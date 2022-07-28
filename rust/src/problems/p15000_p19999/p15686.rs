/// Author: juyoung35

use std::io::{stdin, prelude::*};
const EMPTY: u8 = 0;
const HOUSE: u8 = 1;
const CHICK: u8 = 2;
type Coord = (usize, usize);
fn f(houses: &Vec<Coord>, chicks: &Vec<Coord>, c: usize, idx: usize, m: usize, min: &mut usize, buf: &mut Vec<usize>) {
    if m > 0 {
        return for next in usize::wrapping_add(idx, 1)..c {
            buf.push(next);
            f(houses, chicks, c, next, m - 1, min, buf);
            buf.pop();
        }
    }
    let mut city_cdst = 0;
    for &(xi, yi) in houses {
        let mut cdst = usize::MAX;
        for &chick in &*buf {
            let (xf, yf) = chicks[chick];
            let nd = usize::abs_diff(xf, xi) + usize::abs_diff(yf, yi);
            if nd < cdst { cdst = nd };
        }
        city_cdst += cdst;
    }
    if city_cdst < *min { *min = city_cdst }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<u8>().unwrap();
    let [n, m] = [0; 2].map(|_| read() as usize);
    let mut houses = Vec::new();
    let mut chicks = Vec::new();
    for y in 0..n {
        for x in 0..n {
            match read() {
                EMPTY => (),
                HOUSE => houses.push((y, x)),
                CHICK => chicks.push((y, x)),
                _ => unreachable!(),
            }
        }
    }
    let mut min = usize::MAX;
    let mut buf = Vec::new();
    f(&houses, &chicks, chicks.len(), !0, m, &mut min, &mut buf);
    println!("{}", min);
}