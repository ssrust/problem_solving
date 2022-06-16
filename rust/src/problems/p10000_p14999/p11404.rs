/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
const INF: usize = usize::MAX;
fn floyd_warshall(graph: &mut Vec<Vec<usize>>, n: usize) {
    for m in 0..n {
        for s in 0..n {
            for e in 0..n {
                let nd = usize::saturating_add(graph[s][m], graph[m][e]);
                if nd < graph[s][e] {
                    graph[s][e] = nd;
                }
            }
        }
    }
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let n = read();
    let mut graph = vec![vec![INF; n]; n];
    (0..n).for_each(|i| graph[i][i] = 0);
    let m = read();
    for _ in 0..m {
        let [a, b] = [0; 2].map(|_| read() - 1);
        let c = read();
        graph[a][b] = usize::min(c, graph[a][b]);
    }
    let mut o = BufWriter::new(io::stdout());
    floyd_warshall(&mut graph, n);
    for i in 0..n {
        for j in 0..n {
            write!(o, "{} ", if graph[i][j] != INF { graph[i][j] } else { 0 }).unwrap();
        }
        writeln!(o).unwrap();
    }
}