/// Author: juyoung35

use std::io::{stdin, prelude::*};
use std::fmt::Write;
const NONE: Node = Node::MAX;
type Order = (Vec<Node>, Vec<usize>);
type Node = usize;
#[derive(Clone, Copy, Debug)]
struct TNode {
    lchild: Option<Node>,
    rchild: Option<Node>,
}
impl TNode {
    fn new() -> Self {
        Self {
            lchild: None,
            rchild: None,
        }
    }
    fn update(&mut self, lchild: Option<Node>, rchild: Option<Node>) {
        self.lchild = lchild;
        self.rchild = rchild;
    }
}
fn f(node: Node, idx: usize, btm: usize, top: usize, io: &Order, po: &Order, btree: &mut Vec<TNode>) -> (Node, usize) {
    if idx == 0 { return (node, idx) }
    if io.0[node] < btm || top < io.0[node] { return (node, idx) }
    let (mut lchild, mut rchild) = (None, None);
    let mut ret = (NONE, NONE);
    if io.0[node] < io.0[po.1[idx - 1]] {
        rchild = Some(po.1[idx - 1]);
        if idx == 1 {
            btree[node].update(lchild, rchild);
            return f(po.1[0], 0, btm, top, io, po, btree);
        }
        let nxt = f(po.1[idx - 1], idx - 1, io.0[node] + 1, top, io, po, btree);
        if nxt.1 < io.0[node] {
            if io.0[nxt.0] < btm || top < io.0[nxt.0] {
                btree[node].update(lchild, rchild);
                return (node, idx);
            }
            ret = f(nxt.0, nxt.1, btm, io.0[node] - 1, io, po, btree);
            if ret.1 == 0 && io.0[node] > io.0[ret.0] && io.0[node] - io.0[ret.0] == 1 {
                lchild = Some(nxt.0);
                btree[node].update(lchild, rchild);
                return ret;
            }
            if ret == (nxt.0, nxt.1) {
                btree[node].update(lchild, rchild);
                return ret;
            }
            lchild = Some(nxt.0);
        } else {
            ret = nxt;
        }
    } else {
        lchild = Some(po.1[idx - 1]);
        if idx == 1 && io.0[node] - io.0[po.1[idx - 1]] == 1 { 
            btree[node].update(lchild, rchild);
            return f(po.1[0], 0, btm, top, io, po, btree);
        }
        ret = f(po.1[idx - 1], idx - 1, btm, top - 1, io, po, btree);
        if ret == (po.1[idx - 1], idx - 1) { return ret }
    }
    btree[node].update(lchild, rchild);
    ret
}
fn pre_order(node: Node, btree: &Vec<TNode>, o: &mut String) {
    write!(o, "{} ", node + 1).unwrap();
    if btree[node].lchild.is_some() {
        pre_order(btree[node].lchild.unwrap(), btree, o);
    }
    if btree[node].rchild.is_some() {
        pre_order(btree[node].rchild.unwrap(), btree, o);
    }
}
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_ascii_whitespace();
    let mut read = || it.next().unwrap().parse::<usize>().unwrap();
    let n = read();
    let mut btree = vec![TNode::new(); n];
    let mut in_order = (vec![0; n], vec![0; n]);
    for i in 0..n {
        let x = read() - 1;
        in_order.0[x] = i;
        in_order.1[i] = x;
    }
    let mut post_order = (vec![0; n], vec!{0; n});
    for i in 0..n {
        let x = read() - 1;
        post_order.0[x] = i;
        post_order.1[i] = x;
    }
    let mut o = String::new();
    f(post_order.1[n - 1], n - 1, 0, n - 1, &in_order, &post_order, &mut btree);
    pre_order(post_order.1[n - 1], &btree, &mut o);
    println!("{}", o);
}