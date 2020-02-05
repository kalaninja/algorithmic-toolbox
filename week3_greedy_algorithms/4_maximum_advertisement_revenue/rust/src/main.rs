fn main() {
    let _n = read_line();
    let a: Vec<i32> = read_line().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let b: Vec<i32> = read_line().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", max_sum(a, b));
}

fn max_sum(mut a: Vec<i32>, mut b: Vec<i32>) -> i64 {
    a.sort_unstable_by(|y, x| x.cmp(y));
    b.sort_unstable_by(|y, x| x.cmp(y));

    (0..a.len())
        .map(|i| (a[i], b[i]))
        .fold(0, |mut acc, (a, b)| {
            acc += a as i64 * b as i64;
            acc
        })
}

fn read_line() -> std::io::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[test]
fn case_1() {
    let a = vec![23];
    let b = vec![39];

    assert_eq!(max_sum(a, b), 897);
}

#[test]
fn case_2() {
    let a = vec![1, 3, -5];
    let b = vec![-2, 4, 1];

    assert_eq!(max_sum(a, b), 23);
}