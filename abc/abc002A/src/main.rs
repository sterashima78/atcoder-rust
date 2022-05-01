// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC002A

fn main() {
    input! {
        n: u64,
        m: u64,
    }
    let ans = if n > m { n } else { m };
    println!("{}", ans);
}
