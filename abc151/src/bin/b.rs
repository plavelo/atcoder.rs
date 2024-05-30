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
        nkm: (i32, i32, i32),
        a: [i32; nkm.0 - 1],
    }
    let (n, k, m) = nkm;
    let total: i32 = a.iter().sum();
    let target = n * m;
    println!("{}", if target - total <= 0 { 0 } else if target - total <= k { target - total } else { -1 })
}
