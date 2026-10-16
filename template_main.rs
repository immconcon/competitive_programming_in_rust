#![allow(dead_code)]

use std::{collections::HashMap, io::BufRead};

fn main() {
    //codeforces_main();
    //atcoder_main();
    atcoder_interactive_main();
}

fn codeforces_main() {
    use std::fmt::Write;

    let file: String = std::io::read_to_string(std::io::stdin().lock()).unwrap();
    let mut lines = file.lines();

    macro_rules! read_line {
        (str) => {{
            lines.next().unwrap()
        }};
        ( Vec < $ty_:ty > capacity($cap:expr) ) => {{
            let line = lines.next().unwrap();
            let mut vec: Vec<$ty_> = Vec::with_capacity($cap);
            vec.extend(line.split_ascii_whitespace().map(|x| x.parse::<$ty_>().unwrap()));
            vec
        }};
        ( $($ty_:ty),+ $(,)?) => {{
            let line = lines.next().unwrap();
            let mut bla = line.split_ascii_whitespace();
            ($(
                bla.next().unwrap().parse::<$ty_>().unwrap()
            ),+)
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
    use std::fmt::Write;

    let file: String = std::io::read_to_string(std::io::stdin().lock()).unwrap();
    let mut lines = file.lines();

    macro_rules! read_line {
        (str) => {{
            lines.next().unwrap()
        }};
        ( Vec < $ty_:ty > capacity($cap:expr) ) => {{
            let line = lines.next().unwrap();
            let mut vec: Vec<$ty_> = Vec::with_capacity($cap);
            vec.extend(line.split_ascii_whitespace().map(|x| x.parse::<$ty_>().unwrap()));
            vec
        }};
        ( $($ty_:ty),+ $(,)?) => {{
            let line = lines.next().unwrap();
            let mut bla = line.split_ascii_whitespace();
            ($(
                bla.next().unwrap().parse::<$ty_>().unwrap()
            ),+)
        }};
    }

    let a = read_line!(i32);
    let (b, c) = read_line!(i32, i32);
    let s = read_line!(str);
    let abc = a + b + c;

    let mut buf = String::new();
    // buf.write_str(&bla).unwrap();
    writeln!(&mut buf, "{abc} {s}").unwrap();
    std::io::Write::write(&mut std::io::stdout(), buf.as_bytes()).unwrap();
}

fn atcoder_interactive_main() {
    use std::io::Write;

    let stdin_lock = &mut std::io::stdin().lock();
    let stdout_lock = &mut std::io::stdout().lock();
    let buf = &mut String::new();

    macro_rules! read_line {
        (str) => {{
            read_line(buf, stdin_lock)
        }};
        ( Vec < $ty_:ty > capacity($cap:expr) ) => {{
            let line = read_line(buf, stdin_lock);
            let mut vec: Vec<$ty_> = Vec::with_capacity($cap);
            vec.extend(line.split_ascii_whitespace().map(|x| x.parse::<$ty_>().unwrap()));
            vec
        }};
        ( $($ty_:ty),+ $(,)?) => {{
            let line = read_line(buf, stdin_lock);
            let mut bla = line.split_ascii_whitespace();
            ($(
                bla.next().unwrap().parse::<$ty_>().unwrap()
            ),+)
        }};
    }

    macro_rules! query {
        ($($q:tt)*) => {{
            use std::fmt::Write;

            buf.clear();
            writeln!(buf, $($q)*).unwrap();
            query(buf, stdout_lock, stdin_lock).trim()
        }};
    }

    let (n, _q) = read_line!(u8, usize);
    let unsorted: Vec<u8> = (0..n).map(|diff| b'A' + diff).collect();
    let mut less =
        |probe: u8, seek: u8| match query!("? {} {}", char::from(probe), char::from(seek)) {
            "<" => true,
            _ => false,
        };

    let res = fja_sort(&unsorted, &mut less);

    let s = String::from_utf8(res).unwrap();

    writeln!(stdout_lock, "! {s}").unwrap();
    stdout_lock.flush().unwrap();
}

fn read_line<'a>(buf: &'a mut String, stdin: &mut std::io::StdinLock) -> &'a str {
    buf.clear();
    stdin.read_line(buf).unwrap();
    buf
}

fn query<'a>(
    query_buf: &'a mut String,
    stdout: &mut std::io::StdoutLock,
    stdin: &mut std::io::StdinLock,
) -> &'a str {
    use std::io::Write;

    stdout.write_all(query_buf.as_bytes()).unwrap();
    stdout.flush().unwrap();
    read_query_answer(query_buf, stdin)
}

fn read_query_answer<'a>(buf: &'a mut String, stdin: &mut std::io::StdinLock) -> &'a str {
    buf.clear();
    stdin.read_line(buf).unwrap();
    buf
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

/// This is bullshit and why is it even a thing in practice problems for `atcoder`.
///
/// I just wanted to test API...
fn fja_sort(xs: &[u8], less: &mut impl FnMut(u8, u8) -> bool) -> Vec<u8> {
    let n = xs.len();
    if n < 2 {
        return xs.to_vec();
    }

    let m = n / 2;
    let mut partner = HashMap::new();
    let mut high_values = Vec::new();

    for i in 0..m {
        let (a, b) = (xs[2 * i], xs[2 * i + 1]);
        let (hi, lo) = if less(a, b) { (b, a) } else { (a, b) };
        partner.insert(hi, lo);
        high_values.push(hi);
    }
    let winners = fja_sort(&high_values, less); // sort the larger elements recursively

    // chain = b1, a1, a2, ...; bs[i] = (b_{i+1}, its winner a_{i+1})
    let mut chain: Vec<u8> = partner
        .get(&winners[0])
        .into_iter()
        .chain(winners.iter())
        .copied()
        .collect();

    let mut losers: Vec<(u8, Option<u8>)> =
        winners.iter().map(|&w| (partner[&w], Some(w))).collect();

    if n % 2 == 1 {
        losers.push((xs[n - 1], None)); // leftover: no upper bound
    }

    let total = losers.len();
    // Jacobsthal sequence: 1, 3, 5, 11, 21...
    let (mut prev, mut current, mut pow) = (1usize, 3usize, 4usize);
    while prev < total {
        (prev + 1..=current.min(total)).rev().for_each(|idx| {
            let (this_loser, bound) = losers[idx - 1];
            let high0 = match bound {
                Some(a) => chain.iter().position(|&c| c == a).unwrap(),
                None => chain.len(),
            };

            // Binary search the insertion point for a loser.
            let (mut low, mut high) = (0, high0);
            while low < high {
                let mid = (low + high) / 2;
                if less(chain[mid], this_loser) {
                    low = mid + 1
                } else {
                    high = mid
                }
            }
            chain.insert(low, this_loser);
        });

        // t_k = 2^k - t_{k-1}
        prev = current;
        pow *= 2;
        current = pow - current;
    }
    chain
}
