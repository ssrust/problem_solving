/// Author: XQtbl

use std::cmp::min;
use std::fmt::{Debug, Display};
use std::io::{stdin, stdout};
use std::io::prelude::*;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = b"10\n";
        let output: u32 = get_n(input.as_slice());
        assert_eq!(output, 10)
    }

    #[test]
    fn test_output() {
        let target = 10000;

        let mut buf = Vec::<u8>::new();
        print(&mut buf, target);

        assert_eq!(String::from_utf8(buf).unwrap(), format!("{}\n", target))
    }

    macro_rules! testcase {
        {$tc_id: ident, $input: literal, $output: literal} => (
            #[test]
            fn $tc_id() {
                let input = $input;
                let n = get_n(input.as_slice());

                let mut cache = Vec::new();
                cache.resize(n+1, NOT_EVALED);

                let out = get_least_cnt(n, &mut cache);

                let mut outbuf = Vec::<u8>::new();
                print(&mut outbuf, out);
                assert_eq!(String::from_utf8(outbuf).unwrap(), $output)
            }
        )
    }

    testcase!{testcase1, b"2\n", "1\n"}
    testcase!{testcase2, b"10\n", "3\n"}
    testcase!{testcase3, b"1\n", "0\n"}
}

fn get_n<Output, Reader>(mut cin: Reader) -> Output
    where Reader: BufRead, Output: FromStr, <Output as FromStr>::Err: Debug {
    let mut line = String::new();
    cin.read_line(&mut line);
    line.trim().parse().unwrap()
}

const MAX_N: usize = 10usize.pow(6);
const NOT_EVALED: usize = MAX_N;

fn get_least_cnt(now: usize, cache: &mut  Vec<usize>) -> usize {
    if cache[now] == NOT_EVALED {
        cache[now] = if now == 1 {
            0
        } else {
            let mut cnt = get_least_cnt(now - 1, cache);
            for divider in 2..=3 { // Method 1, 2
                if now % divider == 0 {
                    let tmp = get_least_cnt(now / divider, cache);
                    cnt = min(cnt, tmp);
                }
            }
            cnt + 1
        }
    }
    cache[now]
}

fn print<Writer, Output>(mut cout: Writer, out: Output)
    where Writer: Write, Output: Display {
    writeln!(cout, "{}", out);
}

fn main() {
    let cin = stdin();
    let mut cin  = cin.lock();
    let n = get_n(&mut cin);

    let cout = stdout();
    let mut cout = cout.lock();

    let mut cache = Vec::new();
    cache.resize(n+1, NOT_EVALED);

    let output = get_least_cnt(n, &mut cache);
    print(cout, output);
}