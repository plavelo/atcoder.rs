#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::iter::FromIterator;

#[allow(unused_imports)]
use itertools::{iproduct, Itertools};
#[allow(unused_imports)]
use num_traits::pow;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        nm: (i32, i32),
        pss: [(i32, String); nm.1],
    }
    let mut acmap: HashMap<i32, i32> = HashMap::new();
    let mut acs = 0;
    let mut was = 0;
    for (p, s) in pss {
        if acmap.contains_key(&p) && acmap.get(&p).unwrap() < &0 {
            continue;
        }
        let totalwas = if acmap.contains_key(&p) { acmap.get(&p).unwrap() } else { &0 };
        if s == "AC" {
            acs += 1;
            was += totalwas;
            acmap.insert(p, -1);
        } else if s == "WA" {
            acmap.insert(p, totalwas + 1);
        }
    }
    println!("{} {}", acs, was)
}
