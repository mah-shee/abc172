use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }
    let mut ans = 0;
    let len: usize = s.len();
    for i in 0..len {
        if s.as_bytes()[i] != t.as_bytes()[i] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
