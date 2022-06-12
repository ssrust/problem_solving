/// Author: juyoung35

use std::io::{stdin, prelude::*};
const INF: usize = usize::MAX;
fn get_small_index(n: usize, visited: &Vec<bool>, distance: &Vec<usize>) -> usize {
    let (mut min, mut index) = (INF, 0);
    for i in 0..n {
        if !visited[i] && distance[i] < min {
            (min, index) = (distance[i], i);
        }
    }
    index
}
fn dijkstra(n: usize, start: usize, end: usize, graph: &Vec<Vec<usize>>) -> usize {
    let mut visited = vec![false; n];
    let mut distance = vec![INF; n];
    visited[start] = true;
    for end in 0..n {
        distance[end] = graph[start][end];
    }
    for _ in 1..n - 1 {
        let i = get_small_index(n, &visited, &distance);
        visited[i] = true;
        for j in 0..n {
            if !visited[j] && if let Some(d) = distance[i].checked_add(graph[i][j]) { d } else { continue } < distance[j] {
                distance[j] = distance[i] + graph[i][j];
            }
        }
    }
    distance[end]
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, e] = [read(), read()];
    let mut graph = vec![vec![INF; n]; n];
    (0..n).for_each(|i| graph[i][i] = 0);
    for _ in 0..e {
        let [a, b] = [0; 2].map(|_| read() - 1);
        let c = read();
        graph[a][b] = c;
        graph[b][a] = c;
    }
    let [v1, v2] = [0; 2].map(|_| read() - 1);
    let ans = dijkstra(n, v1, v2, &graph).saturating_add(
        usize::min(
            dijkstra(n, 0, v1, &graph).saturating_add(dijkstra(n, v2, n - 1, &graph)),
            dijkstra(n, 0, v2, &graph).saturating_add(dijkstra(n, v1, n - 1, &graph)),
        )
    );
    println!("{}", if ans != INF { ans as isize } else { -1 });
}