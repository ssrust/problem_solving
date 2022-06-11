/// Author: juyoung35

use std::io::{stdin, prelude::*};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, m] = [read(), read()];
    let mut know_truth: usize = 0;
    (0..read()).for_each(|_| { know_truth |= 1 << read() });
    let mut parties = Vec::with_capacity(m);
    (0..m).for_each(|_| {
        let mut party: usize = 0;
        (0..read()).for_each(|_| party |= 1 << read());
        parties.push(party);
    });
    let mut count = 0;
    'l: while !parties.is_empty() {
        for i in (0..parties.len()).rev() {
            if parties[i] & know_truth != 0 {
                know_truth |= parties[i];
                parties.remove(i);
                continue 'l;
            }
        }
        parties.pop();
        count += 1;
    }
    println!("{}", count);
}