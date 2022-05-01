// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC005A

fn main() {
    input! {
        (x, y): (u64, u64)
    }
    println!("{}", y / x);
}

