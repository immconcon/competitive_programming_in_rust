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

    /// Sum of elements in `0..len`.
    fn prefix_sum(&self, len: usize) -> i64 {
        len as i64 * self.b1.prefix_sum(len) - self.b2.prefix_sum(len)
    }

    /// Sum of elements in `left..right`.
    fn range_sum(&self, left: usize, right: usize) -> i64 {
        self.prefix_sum(right) - self.prefix_sum(left)
    }
}

struct FenwickTree {
    // 1-indexed
    tree: Vec<i64>,
}

impl FenwickTree {
    /// Create a tree of size `n`, all zeros.
    fn new(n: usize) -> Self {
        FenwickTree { tree: vec![0; n] }
    }

    /// Add `delta` to the element at 0-based index `i`.
    fn add(&mut self, idx: usize, delta: i64) {
        let mut mask = idx;
        while mask < self.tree.len() {
            self.tree[mask] += delta;
            mask |= mask + 1;
        }
    }

    /// Sum of elements in `0..idx`
    fn prefix_sum(&self, idx: usize) -> i64 {
        let mut mask = idx;
        let mut sum = 0;
        while mask > 0 {
            sum += self.tree[mask - 1];
            mask &= mask - 1;
        }
        sum
    }

    /// Sum of elements in `left..right`
    fn range_sum(&self, left: usize, right: usize) -> i64 {
        self.prefix_sum(right) - self.prefix_sum(left)
    }
}

struct DiffArray(Vec<i64>);

impl DiffArray {
    fn new(n: usize) -> Self {
        Self(vec![0; n + 1])
    }

    /// Add `val` to every element in the inclusive range `[left..=right]`.
    fn range_add(&mut self, left: usize, right: usize, val: i64) {
        assert!(
            left <= right && right + 1 < self.0.len(),
            "range out of bounds"
        );
        self.0[left] += val;
        self.0[right + 1] -= val;
    }

    /// Build the final array.
    fn build(self) -> Vec<i64> {
        let mut vec = self.0;
        let n = vec.len() - 1;
        let mut sum = 0;
        vec[..n].iter_mut().for_each(|diff| {
            sum += *diff;
            *diff = sum;
        });
        vec.truncate(n);

        vec
    }
}
