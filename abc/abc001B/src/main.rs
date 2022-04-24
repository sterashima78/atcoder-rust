// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC001B

fn main() {
    input! {
        n: i32,
    }
    let val = if n < 100 {
        "00".to_string()
    } else if n < 6000 {
        format!("{:02}", n / 100)
    } else if n < 35000 {
        format!("{:02}", n / 1000 + 50)
    } else if n <= 70000 {
        format!("{:02}", (n / 1000 - 30) / 5 + 80 )
    } else {
        "89".to_string()
    };
    println!("{}", val);
}

