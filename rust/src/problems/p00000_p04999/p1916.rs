/// Author: juyoung35

use std::io::{stdin, prelude::*};
const INF: usize = usize::MAX;
fn get_lowest_vertex(n: usize, visited: &Vec<bool>, d: &Vec<usize>) -> usize {
    let (mut min, mut index) = (INF, 0);
    for i in 0..n {
        if !visited[i] && d[i] < min {
            (min, index) = (d[i], i);
        }
    }
    index
}
fn dijkstra(n: usize, start: usize, end: usize, graph: &Vec<Vec<usize>>) -> usize {
    let mut visited = vec![false; n];
    visited[start] = true;
    let mut d = vec![INF; n];
    (0..n).for_each(|i| d[i] = graph[start][i]);
    for _ in 1..n - 1 {
        let i = get_lowest_vertex(n, &visited, &d);
        visited[i] = true;
        for j in 0..n {
            if visited[j] { continue }
            let nd = if let Some(dst) = usize::checked_add(d[i], graph[i][j]) { dst } else { continue };
            if nd < d[j] { d[j] = nd }
        }
    }
    d[end]
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let n = read();
    let mut graph = vec![vec![INF; n]; n];
    (0..n).for_each(|i| graph[i][i] = 0);
    let m = read();
    for _ in 0..m {
        let [s, e, w] = [read() - 1, read() - 1, read()];
        graph[s][e] = usize::min(w, graph[s][e]);
    }
    let [start, end] = [0; 2].map(|_| read() - 1);
    let ans = dijkstra(n, start, end, &graph);
    println!("{}", ans);
}