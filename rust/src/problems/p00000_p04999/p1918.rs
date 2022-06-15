/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
fn main() {
    let mut level: usize = 0;
    let mut stack = vec![];
    let mut o = BufWriter::new(io::stdout());
    io::stdin().bytes().map(|r| r.unwrap()).take_while(|&b| b != b'\n' && b != b'\r').for_each(|b| match b {
        b'+' | b'-' => {
            level = level.checked_sub(1).unwrap_or(0);
            let len = stack.len();
            for _ in 0..len {
                let &(l, oper) = stack.last().unwrap();
                if l < level { break }
                write!(o, "{}", oper as char).unwrap();
                stack.pop();
            }
            stack.push((level, b));
            level += 1;
        },
        b'*' | b'/' => {
            let len = stack.len();
            for _ in 0..len {
                let &(l, oper) = stack.last().unwrap();
                if l < level { break }
                write!(o, "{}", oper as char).unwrap();
                stack.pop();
            }
            stack.push((level, b))
        },
        b'(' => {
            level += 2;
        },
        b')' => {
            level -= 2;
        },
        a => {
            write!(o, "{}", a as char).unwrap();
        },
    });
    while let Some((_, b)) = stack.pop() {
        write!(o, "{}", b as char).unwrap();
    }
}