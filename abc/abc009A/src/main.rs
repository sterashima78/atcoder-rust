use proconio::input;
fn main() {
    input! {
        n: u64,
    }
    let ans = if n % 2 == 0 { n / 2 } else { n / 2 + 1 };
    println!("{}", ans);
}
