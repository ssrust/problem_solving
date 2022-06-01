use std::io::{self, prelude::*, BufWriter};
const NONE    : u8 = 0b_0000;
const RIGHT   : u8 = 0b_0001;
const LEFT    : u8 = 0b_0010;
const VERTICAL: u8 = 0b_0100;
const MATCHED : u8 = 0b_1000;
fn dfs(n: usize, edge: &mut Vec<Vec<u8>>, y: usize, x: usize, mode: u8, stack: &mut Vec<(u8, (usize, usize))>, visited: &mut Vec<Vec<bool>>) {
    stack.push((mode, (y, x)));
    visited[y][x] = true;
    if edge[y][x] & RIGHT == RIGHT {
        let r = (x + 1) % n;
        let right = edge.get_mut(y).unwrap().get_mut(r).unwrap();
        if *right & LEFT == LEFT && !visited[y][r] {
            *right &= !LEFT;
            dfs(n, edge, y, r, RIGHT, stack, visited);
            // *right |= MATCHED;
        }
    }
    if edge[y][x] & LEFT == LEFT {
        let l = x.checked_sub(1).unwrap_or(n - 1);
        let left = edge.get_mut(y).unwrap().get_mut(l).unwrap();
        if *left & RIGHT == RIGHT && !visited[y][l] {
            *left &= !RIGHT;
            dfs(n, edge, y, l, LEFT, stack, visited);
            // *left |= MATCHED;
        }
    }
    if edge[y][x] & VERTICAL == VERTICAL {
        let v = if y == 0 { 1 } else { 0 };
        let vertical = edge.get_mut(v).unwrap().get_mut(x).unwrap();
        if *vertical & VERTICAL == VERTICAL && !visited[v][x] {
            *vertical &= !VERTICAL;
            dfs(n, edge, v, x, VERTICAL, stack, visited);
            // *vertical |= MATCHED;
        }
    }
}
fn main() {
    let mut input = String::with_capacity(500_000_000);
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    let mut o = BufWriter::new(io::stdout());
    (0..t).for_each(|_| {
        let (n, w) = {
            let mut v = Vec::with_capacity(2);
            lines.next().unwrap().split_ascii_whitespace().for_each(|x| v.push(x.parse::<usize>().unwrap()));
            (v[0], v[1])
        };
        // let mut area = vec![vec![0; n]; 2];
        let area = vec![
            lines.next().unwrap().split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
            lines.next().unwrap().split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>(),
        ];
        let mut edge = vec![vec![NONE; n]; 2];
        // for i in 0..2 {
        //     for j in 0..n {
        //         area[i][j] = read();
        //     }
        // }
        for j in 0..n {
            let u = area[0][j];
            let d = area[1][j];
            if u + d <= w {
                edge[0][j] |= VERTICAL;
                edge[1][j] |= VERTICAL;
            }
            let left = j.checked_sub(1).unwrap_or(n - 1);
            if area[0][left] + u <= w {
                edge[0][left] |= RIGHT;
                edge[0][j] |= LEFT;
            }
            if area[1][left] + d <= w {
                edge[1][left] |= RIGHT;
                edge[1][j] |= LEFT;
            }
        }
        // if n == 1 {
        //     if edge[0][0] & VERTICAL == VERTICAL {
        //         return writeln!(o, "1").unwrap();
        //     } else {
        //         return writeln!(o, "2").unwrap();
        //     }
        // }
        let mut count = 0;
        for y in 0..2 {
            for x in 0..n {
                match edge[y][x] {
                    NONE  => edge[y][x] = MATCHED,
                    RIGHT => {
                        let r = (x + 1) % n;
                        edge[y][x] = MATCHED;
                        edge[y][r] = MATCHED;
                        let rr = (r + 1) % n;
                        let v = if y == 0 { 1 } else { 0 };
                        edge[y][rr] &= !LEFT;
                        edge[v][r]  &= !VERTICAL;
                    },
                    LEFT => {
                        let l = x.checked_sub(1).unwrap_or(n - 1);
                        edge[y][x] = MATCHED;
                        edge[y][l] = MATCHED;
                        let ll = l.checked_sub(1).unwrap_or(n - 1);
                        let v = if y == 0 { 1 } else { 0 };
                        edge[y][ll] &= !RIGHT;
                        edge[v][l]  &= !VERTICAL;
                    },
                    VERTICAL => {
                        let v = if y == 0 { 1 } else { 0 };
                        edge[y][x] = MATCHED;
                        edge[v][x] = MATCHED;
                        let l = x.checked_sub(1).unwrap_or(n - 1);
                        let r = (x + 1) % n;
                        edge[v][l] &= !RIGHT;
                        edge[v][r] &= !LEFT;
                    },
                    _ => continue,
                }
                count += 1;
            }
        }
        for y in 0..2 {
            for x in (0..n).rev() {
                match edge[y][x] {
                    NONE => edge[y][x] = MATCHED,
                    RIGHT => {
                        let r = (x + 1) % n;
                        edge[y][x] = MATCHED;
                        edge[y][r] = MATCHED;
                        let rr = (r + 1) % n;
                        let v = if y == 0 { 1 } else { 0 };
                        edge[y][rr] &= !LEFT;
                        edge[v][r]  &= !VERTICAL;
                    },
                    LEFT => {
                        let l = x.checked_sub(1).unwrap_or(n - 1);
                        edge[y][x] = MATCHED;
                        edge[y][l] = MATCHED;
                        let ll = l.checked_sub(1).unwrap_or(n - 1);
                        let v = if y == 0 { 1 } else { 0 };
                        edge[y][ll] &= !RIGHT;
                        edge[v][l]  &= !VERTICAL;
                    },
                    VERTICAL => {
                        let v = if y == 0 { 1 } else { 0 };
                        edge[y][x] = MATCHED;
                        edge[v][x] = MATCHED;
                        let l = x.checked_sub(1).unwrap_or(n - 1);
                        let r = (x + 1) % n;
                        edge[v][l] &= !RIGHT;
                        edge[v][r] &= !LEFT;
                    },
                    _ => continue,
                }
                count += 1;
            }
        }
        for y in 0..2 {
            for x in 0..n {
                if edge[y][x] == NONE {
                    edge[y][x] = MATCHED;
                    count += 1;
                }
            }
        }
        // write!(o, "{};", count).unwrap();
        let mut visited = vec![vec![false; n]; 2];
        for i in 0..2 {
            for j in 0..n {
                if visited[i][j] || edge[i][j] == MATCHED { continue }
                let mut stack = Vec::with_capacity(n << 1);
                dfs(n, &mut edge, i, j, NONE, &mut stack, &mut visited);
                while let Some((mode, (y, x))) = stack.pop() {
                    if edge[y][x] == MATCHED { continue }
                    count += 1;
                    match mode {
                        NONE => break,
                        RIGHT => {
                            let l = x.checked_sub(1).unwrap_or(n - 1);
                            if edge[y][l] != MATCHED {
                                edge[y][x] = MATCHED;
                                edge[y][l] = MATCHED;
                                let ll = l.checked_sub(1).unwrap_or(n - 1);
                                let v = if y == 0 { 1 } else { 0 };
                                edge[y][ll] &= !RIGHT;
                                edge[v][l]  &= !VERTICAL;
                                let r = (x + 1) % n;
                                edge[y][r] &= !LEFT;
                                edge[v][x] &= !VERTICAL;
                                continue;
                            }
                        },
                        LEFT => {
                            let r = (x + 1) % n;
                            if edge[y][r] != MATCHED {
                                edge[y][x] = MATCHED;
                                edge[y][r] = MATCHED;
                                let rr = (r + 1) % n;
                                let v = if y == 0 { 1 } else { 0 };
                                edge[y][rr] &= !LEFT;
                                edge[v][r]  &= !VERTICAL;
                                let l = x.checked_sub(1).unwrap_or(n - 1);
                                edge[y][l] &= !RIGHT;
                                edge[v][x] &= !VERTICAL;
                                continue;
                            }
                        },
                        VERTICAL => {
                            let v = if y == 0 { 1 } else { 0 };
                            if edge[v][x] != MATCHED {
                                edge[y][x] = MATCHED;
                                edge[v][x] = MATCHED;
                                let l = x.checked_sub(1).unwrap_or(n - 1);
                                let r = (x + 1) % n;
                                edge[y][l] &= !RIGHT;
                                edge[v][l] &= !RIGHT;
                                edge[y][r] &= !LEFT;
                                edge[v][r] &= !LEFT;
                                continue;
                            }
                        },
                        _ => unreachable!(),
                    }
                    if edge[y][x] & RIGHT == RIGHT {
                        let r = (x + 1) % n;
                        if edge[y][r] != MATCHED {
                            edge[y][x] = MATCHED;
                            edge[y][r] = MATCHED;
                            let rr = (r + 1) % n;
                            let v = if y == 0 { 1 } else { 0 };
                            edge[y][rr] &= !LEFT;
                            edge[v][r]  &= !VERTICAL;
                            let l = x.checked_sub(1).unwrap_or(n - 1);
                            edge[y][l] &= !RIGHT;
                            edge[v][x] &= !VERTICAL;
                            continue;
                        }
                    }
                    if edge[y][x] & LEFT == LEFT {
                        let l = x.checked_sub(1).unwrap_or(n - 1);
                        if edge[y][l] != MATCHED {
                            edge[y][x] = MATCHED;
                            edge[y][l] = MATCHED;
                            let ll = l.checked_sub(1).unwrap_or(n - 1);
                            let v = if y == 0 { 1 } else { 0 };
                            edge[y][ll] &= !RIGHT;
                            edge[v][l]  &= !VERTICAL;
                            let r = (x + 1) % n;
                            edge[y][r] &= !LEFT;
                            edge[v][x] &= !VERTICAL;
                            continue;
                        }
                    }
                    if edge[y][x] & VERTICAL == VERTICAL {
                        let v = if y == 0 { 1 } else { 0 };
                        if edge[v][x] != MATCHED {
                            edge[y][x] = MATCHED;
                            edge[v][x] = MATCHED;
                            let l = x.checked_sub(1).unwrap_or(n - 1);
                            let r = (x + 1) % n;
                            edge[y][l] &= !RIGHT;
                            edge[v][l] &= !RIGHT;
                            edge[y][r] &= !LEFT;
                            edge[v][r] &= !LEFT;
                            continue;
                        }
                    }
                }
            }
        }
        writeln!(o, "{} ", count).unwrap();
    });
}