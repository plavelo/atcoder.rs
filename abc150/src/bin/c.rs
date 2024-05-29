#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;
use num_traits::ToPrimitive;

#[fastout]
fn main() {
    input! {
        n: usize,
        ps: [usize; n],
        qs: [usize; n],
    }
    let all: Vec<String> = (1..=n)
        .permutations(n)
        .map(|x| x
            .iter()
            .map(|&y| y.to_string())
            .join("-")
        )
        .collect();
    let pss = ps.iter().map(|p| p.to_string()).join("-");
    let qss = qs.iter().map(|q| q.to_string()).join("-");
    let ppos = all.iter().position(|v| v.to_string() == pss).unwrap().to_i32().unwrap();
    let qpos = all.iter().position(|v| v.to_string() == qss).unwrap().to_i32().unwrap();
    println!("{}", (ppos - qpos).abs());
}
