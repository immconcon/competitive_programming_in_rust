#![allow(dead_code)]

fn main() {
    codeforces_main();
    //atcoder_main();
}

use std::fmt::Write;

fn codeforces_main() {
    let file: String = std::io::read_to_string(std::io::stdin().lock()).unwrap();
    let mut lines = file.lines();

    macro_rules! read_line {
        ( $($ty_:ty),+ $(,)?) => {{
            let line = lines.next().unwrap();
            let mut bla = line.split_ascii_whitespace();
            ($(
                bla.next().unwrap().parse::<$ty_>().unwrap()
            ),+)
        }};
        ( Vec < $ty_:ty > capacity($cap:expr) ) => {{
            let line = lines.next().unwrap();
            let mut vec: Vec<$ty_> = Vec::with_capacity($cap);
            vec.extend(line.split_ascii_whitespace().map(|x| x.parse::<$ty_>().unwrap()));
            vec
        }};
    }

    let testcases_len = read_line!(usize);
    let mut res = Vec::with_capacity(testcases_len);
    //lines().for_each(|line| {
    for _ in 0..testcases_len {
        let (n, mut k) = read_line!(usize, i64);
        let v: Vec<i64> = read_line!(Vec<i64> capacity(n));

        res.push(k);
    }

    let mut buf = String::new();
    res.into_iter().for_each(|bla| {
        // buf.write_str(&bla).unwrap();
        writeln!(&mut buf, "{bla}").unwrap();
    });
    std::io::Write::write(&mut std::io::stdout(), buf.as_bytes()).unwrap();
}

fn atcoder_main() {
    let file: String = std::io::read_to_string(std::io::stdin().lock()).unwrap();
    let mut lines = file.lines();

    macro_rules! read_line {
        ( $($ty_:ty),+ $(,)?) => {{
            let line = lines.next().unwrap();
            let mut bla = line.split_ascii_whitespace();
            ($(
                bla.next().unwrap().parse::<$ty_>().unwrap()
            ),+)
        }};
        ( Vec < $ty_:ty > capacity($cap:expr) ) => {{
            let line = lines.next().unwrap();
            let mut vec: Vec<$ty_> = Vec::with_capacity($cap);
            vec.extend(line.split_ascii_whitespace().map(|x| x.parse::<$ty_>().unwrap()));
            vec
        }};
        (str) => {{
            lines.next().unwrap();
        }};
    }

    let (n, mut k) = read_line!(usize, i64);
    let v: Vec<i64> = read_line!(Vec<i64> capacity(n));

    let mut buf = String::new();
    // buf.write_str(&bla).unwrap();
    writeln!(&mut buf, "{k}").unwrap();
    std::io::Write::write(&mut std::io::stdout(), buf.as_bytes()).unwrap();
}

const MOD: i64 = 1_000_000_000 + 7;

/// Range diff array.
struct RangeFenwick {
    n: usize,
    /// diff[k]
    b1: FenwickTree,
    /// diff[k] * k
    b2: FenwickTree,
}

impl RangeFenwick {
    /// Starts as all zeros.
    fn new(n: usize) -> Self {
        RangeFenwick {
            n,
            b1: FenwickTree::new(n),
            b2: FenwickTree::new(n),
        }
    }

    /// Add `delta` to every element in `left..right`.
    fn range_add(&mut self, left: usize, right: usize, delta: i64) {
        self.b1.add(left, delta);
        self.b2.add(left, delta * left as i64);
        self.b1.add(right, -delta);
        self.b2.add(right, -delta * right as i64);
    }

    /// Sum of elements in [0, p).
    fn prefix_sum(&self, p: usize) -> i64 {
        p as i64 * self.b1.prefix_sum(p) - self.b2.prefix_sum(p)
    }

    /// Sum of elements in [l, r).
    fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum(r) - self.prefix_sum(l)
    }
}

struct FenwickTree {
    // 1-indexed
    tree: Vec<i64>,
}

impl FenwickTree {
    /// Create a tree of size `n`, all zeros.
    fn new(n: usize) -> Self {
        FenwickTree {
            tree: vec![0; n + 1],
        }
    }

    /// Add `delta` to the element at 0-based index `i`.
    fn add(&mut self, i: usize, delta: i64) {
        let mut i = i + 1;
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += lowbit(i);
        }
    }

    /// Sum of elements in `0..i`
    fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= lowbit(i);
        }
        sum
    }

    /// Sum of elements in `l..r`
    fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum(r) - self.prefix_sum(l)
    }
}

/// Computes the lowest set bit in the `i`.
///
/// Equivalent to `1 << i.trailing_zeros()` or with 1.97+ rust `i.isolate_lowest_one()`.
fn lowbit(i: usize) -> usize {
    // i & -i
    i & i.wrapping_neg()
}

struct DiffArray(Vec<i64>);

impl DiffArray {
    fn new(n: usize) -> Self {
        Self(vec![0; n + 1])
    }

    /// Add `delta` to the `[left..right]` range.
    fn add(&mut self, left: usize, right: usize, delta: i64) {
        self.0[left] += delta;
        self.0[right] -= delta;
    }

    /// Add `val` to every element in the inclusive range [l, r]. O(1).
    pub fn range_add(&mut self, l: usize, r: usize, val: i64) {
        assert!(l <= r && r + 1 < self.0.len(), "range out of bounds");
        self.0[l] += val;
        self.0[r + 1] -= val;
    }

    /// Reconstruct the final array via prefix sums. O(n).
    pub fn build(&self) -> Vec<i64> {
        let mut sum = 0;
        self.0[..self.0.len() - 1]
            .iter()
            .map(|&diff| {
                sum += diff;
                sum
            })
            .collect()
    }
}
