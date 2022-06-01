/// Author: juyoung35

use std::io::{self, prelude::*, BufWriter};
fn level(node: usize) -> usize {
    for i in 0..21 {
        if 1 << i <= node + 1 && node + 1 < 1 << i + 1 { return i }
    }
    unreachable!()
}
fn bubble_up_in_max(heap: &mut Vec<isize>, node: usize) {
    if level(node) <= 1 { return }
    let parent = (node - 1) >> 1;
    let grand_parent = (parent - 1) >> 1;
    if heap[grand_parent] < heap[node] {
        heap.swap(grand_parent, node);
        bubble_up_in_max(heap, grand_parent);
    }
}
fn bubble_up_in_min(heap: &mut Vec<isize>, node: usize) {
    if level(node) <= 1 { return }
    let parent = (node - 1) >> 1;
    let grand_parent = (parent - 1) >> 1;
    if heap[grand_parent] > heap[node] {
        heap.swap(grand_parent, node);
        bubble_up_in_min(heap, grand_parent);
    }
}
fn bubble_up(heap: &mut Vec<isize>, node: usize) {
    if node == 0 { return }
    let parent = (node - 1) >> 1;
    if level(node) & 1 == 0 {
        if heap[parent] > heap[node] {
            heap.swap(parent, node);
            bubble_up_in_min(heap, parent);
        } else {
            bubble_up_in_max(heap, node);
        }
    } else {
        if heap[parent] < heap[node] {
            heap.swap(parent, node);
            bubble_up_in_max(heap, parent);
        } else {
            bubble_up_in_min(heap, node);
        }
    }
}
fn insert(heap: &mut Vec<isize>, x: isize) {
    heap.push(x);
    bubble_up(heap, heap.len() - 1);
}
fn tickle_down_in_max(heap: &mut Vec<isize>, node: usize) {
    let &item = if let Some(x) = heap.get(node) { x } else { return };
    let mut max = (node, item);
    let child = ((node + 1) << 1) - 1;
    for idx in child..child+2 {
        if let Some(&x) = heap.get(idx) {
            if x > max.1 { max = (idx, x) }
        }
    }
    let grand_child = ((child + 1) << 1) - 1;
    for idx in grand_child..grand_child+4 {
        if let Some(&x) = heap.get(idx) {
            if x > max.1 { max = (idx, x) }
        }
    }
    match level(max.0) - level(node) {
        2 => {
            if item < max.1 {
                heap.swap(node, max.0);
                let parent_of_grand_child = (max.0 - 1) >> 1;
                if heap[parent_of_grand_child] > heap[max.0] {
                    heap.swap(parent_of_grand_child, max.0);
                }
            }
            tickle_down_in_max(heap, max.0);
        },
        1 => {
            if item < max.1 {
                heap.swap(node, max.0);
            }
        },
        0 => return,
        _ => unreachable!(),
    }
}
fn tickle_down_in_min(heap: &mut Vec<isize>, node: usize) {
    let &item = if let Some(x) = heap.get(node) { x } else { return };
    let mut min = (node, item);
    let child = ((node + 1) << 1) - 1;
    for idx in child..child+2 {
        if let Some(&x) = heap.get(idx) {
            if x < min.1 { min = (idx, x) }
        }
    }
    let grand_child = ((child + 1) << 1) - 1;
    for idx in grand_child..grand_child+4 {
        if let Some(&x) = heap.get(idx) {
            if x < min.1 { min = (idx, x) }
        }
    }
    match level(min.0) - level(node) {
        2 => {
            if item > min.1 {
                heap.swap(node, min.0);
                let parent_of_grand_child = (min.0 - 1) >> 1;
                if heap[parent_of_grand_child] < heap[min.0] {
                    heap.swap(parent_of_grand_child, min.0);
                }
            }
            tickle_down_in_min(heap, min.0);
        },
        1 => {
            if item > min.1 {
                heap.swap(node, min.0);
            }
        },
        0 => return,
        _ => unreachable!(),
    }
}
fn tickle_down(heap: &mut Vec<isize>, node: usize) {
    if level(node) & 1 == 0 {
        tickle_down_in_max(heap, node);
    } else {
        tickle_down_in_min(heap, node);
    }
}
fn deletion(heap: &mut Vec<isize>, node: usize) {
    let last = heap.pop().unwrap();
    if let Some(x) = heap.get_mut(node) { *x = last } else { return }
    tickle_down(heap, node);
}
fn pop_max(heap: &mut Vec<isize>) -> Option<isize> {
    let node = 0;
    let &first = if let Some(x) = heap.first() { x } else { return None };
    let mut max = (node, first);
    let child = ((node + 1) << 1) - 1;
    for idx in child..child+2 {
        if let Some(&x) = heap.get(idx) {
            if x > max.1 { max = (idx, x) }
        }
    }
    let &res = heap.get(max.0).unwrap();
    deletion(heap, max.0);
    Some(res)
}
fn pop_min(heap: &mut Vec<isize>) -> Option<isize> {
    let node = 0;
    let &first = if let Some(x) = heap.first() { x } else { return None };
    let mut min = (node, first);
    let child = ((node + 1) << 1) - 1;
    for idx in child..child+2 {
        if let Some(&x) = heap.get(idx) {
            if x < min.1 { min = (idx, x) }
        }
    }
    let &res = heap.get(min.0).unwrap();
    deletion(heap, min.0);
    Some(res)
}
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap();
    let n: usize = read().parse().unwrap();
    let mut o = BufWriter::new(io::stdout());
    for _ in 0..n {
        let k: usize = read().parse().unwrap();
        let mut heap = vec![];
        for _ in 0..k {
            match read() {
                "I" => insert(&mut heap, read().parse().unwrap()),
                "D" => match read() {
                    "-1" => {
                        pop_min(&mut heap);
                    },
                    "1" => {
                        pop_max(&mut heap);
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
        if heap.is_empty() { writeln!(o, "EMPTY").unwrap() }
        else {
            let max = heap[0];
            writeln!(o, "{} {}", max, pop_min(&mut heap).unwrap()).unwrap();
        }
    }
}