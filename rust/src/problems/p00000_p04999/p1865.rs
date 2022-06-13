/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::fmt::Write;
const INF: isize = isize::MAX;
const NINF: isize = isize::MIN;
fn bellman_ford(n: usize, graph: &Vec<Vec<(isize, usize)>>, o: &mut String) {
    let start = 0;
    let mut distance = vec![INF; n];
    distance[start] = 0;
    for _ in 0..n - 1 {
        for i in 0..n {
            for &(current_distance, j) in &graph[i] {
                let nd = distance[i].saturating_add(current_distance);
                if nd < distance[j] {
                    distance[j] = nd;
                }
            }
        }
    }
    for i in 0..n {
        for &(current_distance, j) in &graph[i] {
            let nd = distance[i].saturating_add(current_distance);
            if nd < distance[j] {
                return writeln!(o, "YES").unwrap();
            }
        }
    }
    writeln!(o, "NO").unwrap();
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let tc = read();
    let mut o = String::new();
    for _ in 0..tc {
        let [n, m, w] = [0; 3].map(|_| read());
        let mut graph = vec![vec![]; n];
        for _ in 0..m {
            let [u, v] = [0; 2].map(|_| read() - 1);
            let weight = read() as isize;
            graph[u].push((weight, v));
            graph[v].push((weight, u));
        }
        for _ in 0..w {
            let [u, v] = [0; 2].map(|_| read() - 1);
            graph[u].push((-(read() as isize), v));
        }
        for i in 0..n {
            graph[i].sort_unstable();
        }
        bellman_ford(n, &graph, &mut o);
    }
    println!("{}", o);
}