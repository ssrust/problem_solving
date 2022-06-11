/// Author: juyoung35

use std::io::{stdin, prelude::*};
fn dfs(graph: &Vec<Vec<(usize, usize)>>, visited: &mut Vec<bool>, node: usize, diameter: usize, max: &mut (usize, usize)) {
    let mut leap = true;
    visited[node] = true;
    for &(next, distance) in &graph[node] {
        if !visited[next] {
            leap = false;
            dfs(graph, visited, next, diameter + distance, max);
        }
    }
    visited[node] = false;
    if leap {
        if diameter > max.1 { *max = (node, diameter) }
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>();
    let v: usize = read().unwrap();
    let mut graph = vec![vec![]; v + 1];
    for _ in 0..v {
        let i = read().unwrap();
        loop {
            let [w, d] = [if let Ok(x) = read() { x } else { break }, read().unwrap()];
            graph[i].push((w, d));
        }
    }
    let mut visited = vec![false; v + 1];
    let mut max = (0, 0);
    dfs(&graph, &mut visited, 1, 0, &mut max);
    let mut visited = vec![false; v + 1];
    dfs(&graph, &mut visited, max.0, 0, &mut max);
    println!("{}", max.1);
}