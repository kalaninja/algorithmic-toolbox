fn main() {
    let _n = read_line();
    let vec = read_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let result = if solve(vec) { 1 } else { 0 };
    println!("{}", result);
}

fn solve(vec: Vec<u32>) -> bool {
    let sum = vec.iter().sum::<u32>();
    if sum % 3 != 0 { return false; }

    let n = (sum / 3 + 1) as usize;
    let mut s = vec![vec![false; n]; n];
    s[0][0] = true;

    for v in vec {
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if s[i][j] {
                    let new_j = j + v as usize;
                    let new_i = i + v as usize;
                    if new_j < n { s[i][new_j] = true; }
                    if new_i < n { s[new_i][j] = true; }
                }
            }
        }

        if s[n - 1][n - 1] { return true; }
    }

    s[n - 1][n - 1]
}

#[test]
fn test_case_1() {
    assert_eq!(solve(vec![3, 3, 3, 3]), false);
}

#[test]
fn test_case_2() {
    assert_eq!(solve(vec![40]), false);
}

#[test]
fn test_case_3() {
    assert_eq!(solve(vec![17, 59, 34, 57, 17, 23, 67, 1, 18, 2, 59]), true);
}

#[test]
fn test_case_4() {
    assert_eq!(solve(vec![1, 2, 3, 4, 5, 5, 7, 7, 8, 10, 12, 19, 25]), true);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}