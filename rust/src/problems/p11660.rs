/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, m] = [read(), read()];
    let mut sum = vec![vec![0; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            sum[i][j] += read();
            sum[i][j] += sum[i - 1][j    ];
            sum[i][j] += sum[i    ][j - 1];
            sum[i][j] -= sum[i - 1][j - 1];
        }
    }
    let mut o = BufWriter::new(io::stdout());
    for _ in 0..m {
        let [x1, y1, x2, y2] = [0; 4].map(|_| read());
        let mut ans = 0;
        ans += sum[x2    ][y2    ];
        ans += sum[x1 - 1][y1 - 1];
        ans -= sum[x2    ][y1 - 1];
        ans -= sum[x1 - 1][y2    ];
        writeln!(o, "{}", ans).unwrap();
    }
}