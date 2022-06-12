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
fn dijkstra(n: usize, root: usize, graph: &Vec<Vec<usize>>) -> usize {
    let mut round = vec![0; n];
    dijkstra_with_start(n, root, graph, &mut round);
    dijkstra_with_end(n, root, graph, &mut round);
    round.into_iter().max().unwrap()
}
fn dijkstra_with_start(n: usize, start: usize, graph: &Vec<Vec<usize>>, round: &mut Vec<usize>) {
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
    for i in 0..n {
        round[i] += distance[i];
    }
}
fn dijkstra_with_end(n: usize, end: usize, graph: &Vec<Vec<usize>>, round: &mut Vec<usize>) {
    let mut visited = vec![false; n];
    let mut distance = vec![INF; n];
    visited[end] = true;
    for start in 0..n {
        distance[start] = graph[start][end];
    }
    for _ in 1..n - 1 {
        let i = get_small_index(n, &visited, &distance);
        visited[i] = true;
        for j in 0..n {
            if !visited[j] && if let Some(d) = distance[i].checked_add(graph[j][i]) { d } else { continue } < distance[j] {
                distance[j] = distance[i] + graph[j][i];
            }
        }
    }
    for i in 0..n {
        round[i] += distance[i];
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, m, x] = [0; 3].map(|_| read());
    let mut graph = vec![vec![INF; n]; n];
    (0..n).for_each(|i| graph[i][i] = 0);
    (0..m).for_each(|_| if let Some(t) = graph.get_mut(read() - 1).and_then(|dst| dst.get_mut(read() - 1)) { *t = read() });
    let ans = dijkstra(n, x - 1, &graph);
    println!("{}", ans);
}