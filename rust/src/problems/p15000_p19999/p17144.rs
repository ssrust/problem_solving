/// Author: juyoung35

use std::io::stdin;
use std::convert::TryInto;
const NONE: usize = usize::MAX;
const CLEANER: usize = usize::MAX;
const THRESHOLD: usize = 5;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let [r, c, t]: [usize; 3] = input.trim_end().split_ascii_whitespace().take(3).map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>().try_into().unwrap();
    let mut room = vec![vec![0; c]; r];
    let (mut up, mut down) = (NONE, NONE);
    for y in 0..r {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        input.trim_end().split_ascii_whitespace().take(c).enumerate().for_each(|(x, item)|
            room[y][x] = if item != "-1" { item.parse::<usize>().unwrap() }
            else {
                if up == NONE { up = y }
                else { down = y }
                CLEANER
            }
        );
    }
    let mut stack = Vec::new();
    for _ in 0..t {
        stack.clear();
        for y in 0..r {
            for x in 0..c {
                let amount = room[y][x];
                if amount >= THRESHOLD && amount != CLEANER {
                    stack.push((amount, (y, x)));
                }
            }
        }
        while let Some((amount, (y, x))) = stack.pop() {
            let quantity = amount / 5;
            if x > 0 {
                if room[y][x - 1] != CLEANER {
                    room[y][x - 1] += quantity;
                    room[y][x] -= quantity;
                }
            }
            if x < c - 1 {
                if room[y][x + 1] != CLEANER {
                    room[y][x + 1] += quantity;
                    room[y][x] -= quantity;
                }
            }
            if y > 0 {
                if room[y - 1][x] != CLEANER {
                    room[y - 1][x] += quantity;
                    room[y][x] -= quantity;
                }
            }
            if y < r - 1 {
                if room[y + 1][x] != CLEANER {
                    room[y + 1][x] += quantity;
                    room[y][x] -= quantity;
                }
            }
        }
        for y in (0..up).rev() {
            room[y + 1][0] = room[y][0];
        }
        room[up][0] = 0;
        for x in 1..c {
            room[0][x - 1] = room[0][x];
        }
        for y in 1..=up {
            room[y - 1][c - 1] = room[y][c - 1];
        }
        for x in (0..c - 1).rev() {
            room[up][x + 1] = room[up][x];
        }
        room[up][0] = CLEANER;
        for y in down + 1..r {
            room[y - 1][0] = room[y][0];
        }
        room[down][0] = 0;
        for x in 1..c {
            room[r - 1][x - 1] = room[r - 1][x];
        }
        for y in (up..r - 1).rev() {
            room[y + 1][c - 1] = room[y][c - 1];
        }
        for x in (0..c - 1).rev() {
            room[down][x + 1] = room[down][x];
        }
        room[down][0] = CLEANER;
    }
    let mut sum = 0;
    for y in 0..r {
        for x in 0..c {
            let amount = room[y][x];
            if amount != CLEANER { sum += amount }
        }
    }
    println!("{}", sum);
}