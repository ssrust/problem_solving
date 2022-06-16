/// Author: juoyung35

use std::io::{stdin, prelude::*};
fn dfs(graph: &mut Vec<Vec<usize>>, nodes: &mut Vec<usize>, n: usize, i: usize, count: &mut usize) {
    if i == n { return *count += 1 }
    for node in 0..n - i {
        let j = nodes[node];
        if graph[i][j] > 0 { continue }
        nodes.remove(node);
        for k in 0..n {
            graph[i][k] += 1;
            graph[k][j] += 1;
        }
        let (mut x, mut y) = (0, 0);
        (x, y) = (i, j);
        while x > 0 && y > 0 {
            x -= 1;
            y -= 1;
            graph[x][y] += 1;
        }
        (x, y) = (i, j);
        while x < n && y < n {
            graph[x][y] += 1;
            x += 1;
            y += 1;
        }
        (x, y) = (i, j + 1);
        while x > 0 && y < n {
            x -= 1;
            graph[x][y] += 1;
            y += 1;
        }
        (x, y) = (i + 1, j);
        while y > 0 && x < n {
            y -= 1;
            graph[x][y] += 1;
            x += 1;
        }
        dfs(graph, nodes, n, i + 1, count);
        nodes.insert(node, j);
        for k in 0..n {
            graph[i][k] -= 1;
            graph[k][j] -= 1;
        }
        let (mut x, mut y) = (0, 0);
        (x, y) = (i, j);
        while x > 0 && y > 0 {
            x -= 1;
            y -= 1;
            graph[x][y] -= 1;
        }
        (x, y) = (i, j);
        while x < n && y < n {
            graph[x][y] -= 1;
            x += 1;
            y += 1;
        }
        (x, y) = (i, j + 1);
        while x > 0 && y < n {
            x -= 1;
            graph[x][y] -= 1;
            y += 1;
        }
        (x, y) = (i + 1, j);
        while y > 0 && x < n {
            y -= 1;
            graph[x][y] -= 1;
            x += 1;
        }
    }
}
fn main() {
    let n: usize = stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    let mut graph = vec![vec![0; n]; n];
    let mut nodes = (0..n).collect();
    let mut count = 0;
    dfs(&mut graph, &mut nodes, n, 0, &mut count);
    println!("{}", count);
}