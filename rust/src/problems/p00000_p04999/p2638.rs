/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::collections::VecDeque;
type Cheese = u8;
type Coord = (usize, usize);
type Grid = Vec<Vec<Cheese>>;
type Visited = Vec<Vec<bool>>;
type Deq = VecDeque<Coord>;
const NONE  : Cheese = 0b_0000_0000;
const AIR   : Cheese = 0b_0000_0001;
const CHEESE: Cheese = 0b_0000_0010;
const VOID  : Cheese = 0b_0000_0100;
const MELT  : Cheese = 0b_0000_1000;
const RIGHT : Cheese = 0b_0001_0000;
const LEFT  : Cheese = 0b_0010_0000;
const DOWN  : Cheese = 0b_0100_0000;
const UP    : Cheese = 0b_1000_0000;
const VOID_CANDIDATE: Cheese = AIR | VOID;
const ALL_DIRECTION: Cheese = RIGHT | LEFT | DOWN | UP;
fn update(grid: &mut Grid, deq: &mut Deq, y: usize, x: usize, direction: Cheese) -> bool {
    let mut ret = false;
    if grid[y][x] & MELT != MELT && grid[y][x] & ALL_DIRECTION != ALL_DIRECTION && grid[y][x] & direction == direction {
        grid[y][x] |= MELT;
        deq.push_back((y, x));
        ret = true;
    }
    grid[y][x] &= !direction;
    ret
}
fn dfs(grid: &mut Grid, visited: &mut Visited, deq: &mut Deq, n: usize, m: usize, y: usize, x: usize) -> usize {
    let mut ret = 0;
    grid[y][x] &= !VOID;
    if y > 0 && grid[y - 1][x] & CHEESE == CHEESE {
        if update(grid, deq, y - 1, x, DOWN) { ret += 1 }
    }
    if x > 0 && grid[y][x - 1] & CHEESE == CHEESE {
        if update(grid, deq, y, x - 1, RIGHT) { ret += 1 }
    }
    if y < n - 1 && grid[y + 1][x] & CHEESE == CHEESE {
        if update(grid, deq, y + 1, x, UP) { ret += 1 }
    }
    if x < m - 1 && grid[y][x + 1] & CHEESE == CHEESE {
        if update(grid, deq, y, x + 1, LEFT) { ret += 1 }
    }
    visited[y][x] = true;
    if y > 0 && !visited[y - 1][x] && grid[y - 1][x] & VOID_CANDIDATE == VOID_CANDIDATE {
        ret += dfs(grid, visited, deq, n, m, y - 1, x);
    }
    if x > 0 && !visited[y][x - 1] && grid[y][x - 1] & VOID_CANDIDATE == VOID_CANDIDATE {
        ret += dfs(grid, visited, deq, n, m, y, x - 1);
    }
    if y < n - 1 && !visited[y + 1][x] && grid[y + 1][x] & VOID_CANDIDATE == VOID_CANDIDATE {
        ret += dfs(grid, visited, deq, n, m, y + 1, x);
    }
    if x < m - 1 && !visited[y][x + 1] && grid[y][x + 1] & VOID_CANDIDATE == VOID_CANDIDATE {
        ret += dfs(grid, visited, deq, n, m, y, x + 1);
    }
    ret
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let [n, m] = [0; 2].map(|_| read());
    let mut grid = vec![vec![ALL_DIRECTION; m]; n];
    let mut visited = vec![vec![false; m]; n];
    let mut deq = VecDeque::new();
    for y in 0..n {
        for x in 0..m {
            let mut res = NONE;
            match read() {
                0 => {
                    if y == 0 || x == 0 || y == n - 1 || x == m - 1 || (y > 0 && grid[y - 1][x] & VOID_CANDIDATE == AIR) || (x > 0 && grid[y][x - 1] & VOID_CANDIDATE == AIR) {
                        res |= AIR;
                        if y > 0 && grid[y - 1][x] & CHEESE == CHEESE { res |= UP }
                        if x > 0 && grid[y][x - 1] & CHEESE == CHEESE { res |= LEFT }
                        dfs(&mut grid, &mut visited, &mut deq, n, m, y, x);
                    } else {
                        res |= VOID_CANDIDATE;
                        if y > 0 && grid[y - 1][x] & CHEESE == CHEESE {
                            res |= UP;
                            grid[y - 1][x] |= DOWN;
                        }
                        if x > 0 && grid[y][x - 1] & CHEESE == CHEESE {
                            res |= LEFT;
                            grid[y][x - 1] |= RIGHT;
                        }
                    }
                },
                1 => {
                    res |= CHEESE;
                    res |= ALL_DIRECTION;
                    if y > 0 {
                        grid[y - 1][x] |= DOWN;
                        if grid[y - 1][x] & CHEESE == CHEESE || grid[y - 1][x] & VOID_CANDIDATE == VOID_CANDIDATE {
                            res |= UP;
                        } else {
                            res &= !UP;
                        }
                    }
                    if x > 0 {
                        grid[y][x - 1] |= RIGHT;
                        if grid[y][x - 1] & CHEESE == CHEESE || grid[y][x - 1] & VOID_CANDIDATE == VOID_CANDIDATE {
                            res |= LEFT;
                        } else {
                            res &= !LEFT;
                        }
                    }
                    if y > 0 && x > 0 && grid[y - 1][x] & VOID_CANDIDATE == AIR && grid[y][x - 1] & VOID_CANDIDATE == AIR {
                        res |= MELT;
                        deq.push_back((y, x));
                    }
                },
                _ => unreachable!(),
            }
            grid[y][x] = res;
        }
    }
    let mut count = 0;
    let mut level = deq.len();
    let mut next = 0;
    while let Some((y, x)) = deq.pop_front() {
        if y > 0 {
            if grid[y - 1][x] & CHEESE == CHEESE && update(&mut grid, &mut deq, y - 1, x, DOWN) {
                next += 1;
            } else { grid[y - 1][x] &= !DOWN }
        }
        if x > 0 {
            if grid[y][x - 1] & CHEESE == CHEESE && update(&mut grid, &mut deq, y, x - 1, RIGHT) {
                next += 1;
            } else { grid[y][x - 1] &= !RIGHT }
        }
        if y < n - 1 {
            if grid[y + 1][x] & CHEESE == CHEESE && update(&mut grid, &mut deq, y + 1, x, UP) {
                next += 1;
            } else { grid[y + 1][x] &= !UP }
        }
        if x < m - 1 {
            if grid[y][x + 1] & CHEESE == CHEESE && update(&mut grid, &mut deq, y, x + 1, LEFT) {
                next += 1;
            } else { grid[y][x + 1] &= !LEFT }
        }
        grid[y][x] = AIR;
        next += dfs(&mut grid, &mut visited, &mut deq, n, m, y, x);
        level -= 1;
        if level == 0 {
            level = next;
            next = 0;
            count += 1;
        }
    }
    println!("{}", count);
}