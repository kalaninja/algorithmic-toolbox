use std::str::FromStr;
use std::fmt::Debug;

fn main() {
    let size = read_line()[0];
    let weights = read_line();

    let result = solve(size, weights);
    println!("{}", result);
}

fn solve(size: usize, weights: Vec<u32>) -> u32 {
    let n = size + 1;
    let m = weights.len() + 1;
    let mut s = vec![vec![0; n]; m];

    for i in 1..m {
        for j in 1..n {
            let cur_weight = weights[i - 1];
            if cur_weight > j as u32 {
                s[i][j] = s[i - 1][j]
            } else {
                s[i][j] = s[i - 1][j].max(s[i - 1][j - cur_weight as usize] + cur_weight);
            }
        }
    }

    s[m - 1][n - 1]
}

#[test]
fn test_case_1() {
    assert_eq!(solve(10, vec![1, 4, 8]), 9);
}

fn read_line<T: FromStr>() -> Vec<T> where <T as FromStr>::Err: Debug {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}