/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
fn main() {
    let mut input = Vec::with_capacity(1_000_000);
    let mut bytes = io::stdin().bytes().map(|r| r.unwrap());
    bytes.by_ref().take_while(|&b| b != b'\n').for_each(|b| input.push(b));
    let mut explosion = Vec::with_capacity(36);
    bytes.take_while(|&b| b != b'\n').for_each(|b| explosion.push(b));
    let len = explosion.len();
    let mut output = Vec::with_capacity(1_000_000);
    let mut stack = Vec::new();
    for i in 0..input.len() {
        let j = usize::wrapping_add(*stack.last().unwrap_or(&!0), 1);
        if input[i] == explosion[j] {
            if j == 0 { stack.push(!0) }
            if j == len - 1 {
                stack.pop();
            } else {
                *if let Some(ptr) = stack.last_mut() { ptr } else {
                    stack.push(0);
                    continue;
                } = j;
            }
        } else {
            if input[i] == explosion[0] {
                stack.push(0);
            }
            else {
                output.push(input[i]);
                stack.push(!0);
            }
        }
    }
    if stack.is_empty() { return println!("FRULA") }
    output.reverse();
    let mut o = BufWriter::new(io::stdout());
    for i in stack {
        if i == !0 {
            write!(o, "{}", output.pop().unwrap() as char).unwrap();
            continue;
        }
        for j in 0..=i {
            write!(o, "{}", explosion[j] as char).unwrap();
        }
    }
}