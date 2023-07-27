use proconio::input;

fn main() {
    input! {
        a: usize,
        bc: (usize, usize),
        s: String,
    };
    println!("{} {}", a + bc.0 + bc.1, s);
}
