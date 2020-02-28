fn main() {
    let _n = read_line();
    let a = read_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let _m = read_line();
    let b = read_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = lcs(a, b);
    println!("{}", result);
}

fn lcs(a: Vec<i32>, b: Vec<i32>) -> u32 {
    let n = a.len() + 1;
    let m = b.len() + 1;
    let mut d = vec![vec![0; m]; n];

    for i in 1..n {
        for j in 1..m {
            if a[i - 1] == b[j - 1] {
                d[i][j] = d[i-1][j-1] + 1;
            } else {
                d[i][j] = d[i - 1][j].max(d[i][j - 1]);
            }
        }
    }

    d[n - 1][m - 1]
}

#[test]
fn test_case_1() {
    let result = lcs(vec![2, 7, 5], vec![2, 5]);
    assert_eq!(result, 2);
}

#[test]
fn test_case_2() {
    let result = lcs(vec![7], vec![1, 2, 3, 4]);
    assert_eq!(result, 0);
}

#[test]
fn test_case_3() {
    let result = lcs(vec![2, 7, 8, 3], vec![5, 2, 8, 7]);
    assert_eq!(result, 2);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}