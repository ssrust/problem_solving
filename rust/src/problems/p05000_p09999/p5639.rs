/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::fmt::Write;
const NONE: usize = usize::MAX;
fn f(key: usize, idx: usize, btm: usize, top: usize, po: &Vec<usize>, o: &mut String) -> usize {
    let mut ret = NONE;
    if key < btm || top < key { return idx }
    if if let Some(&k) = po.get(idx + 1) { k } else {writeln!(o, "{}", key).unwrap(); return idx } < key {
        let nxt = f(po[idx + 1], idx + 1, btm, key - 1, po, o);
        if nxt != NONE {
            ret = f(if let Some(&k) = po.get(nxt) { k } else { writeln!(o, "{}", key).unwrap(); return idx }, nxt, key + 1, top, po, o);
        } else {
            ret = nxt;
        }
    } else {
        ret = f(if let Some(&k) = po.get(idx + 1) { k } else { writeln!(o, "{}", key).unwrap(); return idx }, idx + 1, key + 1, top, po, o);
    }
    writeln!(o, "{}", key).unwrap();
    ret
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let it = input.split_ascii_whitespace();
    let mut po = Vec::new();
    it.for_each(|s| po.push(s.parse::<usize>().unwrap()));
    let mut o = String::new();
    f(*po.first().unwrap(), 0, 0, usize::MAX, &po, &mut o);
    print!("{}", o);
}