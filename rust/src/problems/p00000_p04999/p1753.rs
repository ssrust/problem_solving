/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::fmt::Write;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
const INF: usize = usize::MAX;
fn dijkstra(n: usize, start: usize, graph: &Vec<Vec<(usize, usize)>>, o: &mut String) {
    let mut visited = vec![false; n];
    let mut distance = vec![INF; n];
    distance[start] = 0;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), start));
    while let Some((current_distance, i)) = pq.pop() {
        if visited[i] { continue }
        visited[i] = true;
        let cd = current_distance.0;
        if distance[i] < cd { continue }
        for &(next_distance, j) in &graph[i] {
            let nd = if let Some(d) = cd.checked_add(next_distance) { d } else { continue };
            if nd < distance[j] {
                distance[j] = nd;
                pq.push((Reverse(nd), j));
            }
        }
    }
    for i in 0..n {
        if distance[i] != INF { writeln!(o, "{}", distance[i]).unwrap() }
        else { writeln!(o, "INF").unwrap() }
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, e] = [read(), read()];
    let mut graph = vec![vec![]; n];
    let start = read() - 1;
    for _ in 0..e {
        let [u, v] = [0; 2].map(|_| read() - 1);
        let w = read();
        graph[u].push((w, v));
    }
    for i in 0..n {
        graph[i].sort_unstable();
    }
    let mut o = String::new();
    dijkstra(n, start, &graph, &mut o);
    println!("{}", o);
}