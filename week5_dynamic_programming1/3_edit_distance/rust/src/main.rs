fn main() {
    let a = read_line();
    let b = read_line();

    let d = editing_distance(a.trim(), b.trim());
    println!("{}", d);
}

fn editing_distance(a_str: &str, b_str: &str) -> u32 {
    let a = a_str.chars().collect::<Vec<_>>();
    let b = b_str.chars().collect::<Vec<_>>();

    let n = a.len() + 1;
    let m = b.len() + 1;

    let mut d = vec![vec![0; m]; n];
    (1..n).for_each(|x| d[x][0] = x as u32);
    (1..m).for_each(|x| d[0][x] = x as u32);

    for i in 1..n {
        for j in 1..m {
            let min = d[i - 1][j].min(d[i][j - 1]);
            d[i][j] =
                if a[i - 1] == b[j - 1] {
                    d[i - 1][j - 1].min(min + 1)
                } else {
                    d[i - 1][j - 1].min(min) + 1
                };
        }
    }

    d[n - 1][m - 1]
}

#[test]
fn test_case_1() {
    let result = editing_distance("ab", "ab");
    assert_eq!(result, 0);
}

#[test]
fn test_case_2() {
    let result = editing_distance("short", "ports");
    assert_eq!(result, 3);
}

#[test]
fn test_case_3() {
    let result = editing_distance("editing", "distance");
    assert_eq!(result, 5);
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}