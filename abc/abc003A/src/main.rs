// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC003A

fn main() {
    input! {
        n: u64,
    }
    let ans = (1..=n).collect::<Vec<u64>>().iter().sum::<u64>() * 10000 / n;
    println!("{}", ans);
}

