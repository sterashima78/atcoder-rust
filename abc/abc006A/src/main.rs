// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC004A

fn main() {
    input! {
        n: u64,
    }
    let ans = if n % 3 == 0 {
        "YES"
    } else if n.to_string().find("3").is_some() {
        "YES"
    } else {
        "NO"
    };
    println!("{}", ans);
}
