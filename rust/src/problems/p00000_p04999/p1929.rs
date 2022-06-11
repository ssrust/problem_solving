/// Author: XQtbl

pub use improved::*;

mod legacy {
    use std::io::stdin;
    use std::iter::*;

    fn get_m_n() -> (usize, usize) {
        let cin = stdin();
        let mut line = String::new();
        cin.read_line(&mut line).unwrap();
        line.pop();
        let mut it_ints = line.split(' ').map(|n| {
            n.parse::<usize>().unwrap()
        });
        (it_ints.next().unwrap(), it_ints.next().unwrap())
    }

    fn get_primes(from: usize, to: usize) -> Box<dyn Iterator<Item=usize>> {
        const MAX_IDX: usize = 1000001;
        static mut NOT_PRIMES: [bool; MAX_IDX] = [false; MAX_IDX];

        unsafe { NOT_PRIMES[1] = true; }

        {
            let mut i = 2;
            while i * i <= to {
                let mut j = i + i;
                while j <= to {
                    unsafe { NOT_PRIMES[j] = true };
                    j += i;
                }
                i += 1;
            }
        }
        unsafe {
            Box::new(NOT_PRIMES[from..=to].iter()
                .enumerate()
                .filter(|(_, is_not_prime)| !**is_not_prime)
                .map(move |(idx, _)| idx + from))
        }
    }

    pub fn main() {
        let (m, n) = get_m_n();
        for prime in get_primes(m, n) {
            println!("{}", prime);
        }
    }
}

mod improved {
    use std::io::{stdout, prelude::*, stdin};
    use std::iter::*;
    use std::cmp::max;
    use std::fmt::{Debug, Display};
    use std::ops::RangeInclusive;
    use std::str::FromStr;

    fn get_inputs<T: FromStr>() -> Vec<T>
        where <T as FromStr>::Err: Debug {
        let cin = stdin();
        let mut inputs = String::new();
        cin.lock().read_to_string(&mut inputs).unwrap();
        inputs.trim().split_whitespace().map(|n| {
            n.parse::<T>().expect("Failed to parse")
        }).collect()
    }

    fn println<T>(data: &[T])
        where T: Display {
        let cout = stdout();
        let mut cout = cout.lock();
        for datum in data {
            writeln!(&mut cout, "{}", datum).unwrap();
        }
    }

    fn get_primes(range: RangeInclusive<usize>) -> Vec<usize> {
        // i: 2..=sqrt(to), stride=1
        // j: i+i..=to, stride=i ==> 2i, 3i, 4i, ...
        let (from, to) = range.into_inner();
        let mut is_primes = vec![true; to + 1];

        for i in (2..).take_while(|i| i * i <= to) {
            if let Some(false) = is_primes.get(i) { // Skip if i is not prime
                continue;
            }
            for j in (2..).map(|n| n * i).take_while(|&j| j <= to) {
                is_primes[j] = false;
            }
        }
        is_primes.iter().enumerate().skip(max(from, 2)).filter(|(_, &is_prime)| is_prime).map(|(idx, _)| idx).collect()
        // let cout = stdout();
        // let mut cout = BufWriter::new(cout.lock());
        // for (idx, &is_prime) in  {
        //     if is_prime {
        //         writeln!(&mut cout, "{}", idx).unwrap();
        //     }
        // }
    }

    pub fn main() {
        let range: Vec<usize> = get_inputs();
        println(get_primes(range[0]..=range[1]).as_ref());
    }
}