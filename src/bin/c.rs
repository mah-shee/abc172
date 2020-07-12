use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut a_fold: Vec<usize> = vec![0; n + 1];
    let mut b_fold: Vec<usize> = vec![0; m + 1];
    for i in 0..n {
        a_fold[i + 1] = a[i] + a_fold[i];
    }
    for i in 0..m {
        b_fold[i + 1] = b[i] + b_fold[i];
    }
    let mut ans = 0;
    let mut last = k;
    let mut b_best = m;
    for i in 0..(n + 1) {
        if last < a_fold[i] {
            break;
        }
        last = last - a_fold[i];

        while b_fold[b_best] > last {
            b_best -= 1;
        }
        ans = std::cmp::max(ans, i + b_best);
        last = k;
    }
    println!("{}", ans);
}
