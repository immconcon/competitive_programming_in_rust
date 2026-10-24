#![allow(dead_code)]

use std::io::BufRead;

fn main() {
    //codeforces_main();
    //atcoder_main();
    interactive_main();
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

// I guess atcoder and codeforces interactive problems are close enough
// that there's no need in splitting them into two different templates.
// anyway, now template is for this:
// https://codeforces.com/gym/101021/problem/1
fn interactive_main() {
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

    let mut target_is_less_than = |guess| query!("{}", guess) == "<";
    let mut left = 1;
    let mut right = 1_000_000;
    while left < right {
        let mid = (left + right + 1) / 2;
        if target_is_less_than(mid) {
            right = mid - 1;
        } else {
            left = mid;
        }
    }

    writeln!(stdout_lock, "! {left}").unwrap();
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
