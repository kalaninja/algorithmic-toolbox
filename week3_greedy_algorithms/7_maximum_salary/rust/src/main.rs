use std::io;

fn main() {
    let _n = read_line();
    let a = read_line().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", max_salary(a));
}

fn max_salary(mut a: Vec<u32>) -> String {
    fn concat(a: u32, b: u32) -> u64 {
        format!("{}{}", a, b).parse().unwrap()
    };

    a.sort_by(|&x, &y| concat(y, x).cmp(&concat(x, y)));

    a.iter().fold(String::new(), |x, y| format!("{}{}", x, y))
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[test]
fn case_1() {
    let a = vec![21, 2];
    assert_eq!(max_salary(a), 221);
}

#[test]
fn case_2() {
    let a = vec![9, 4, 6, 1, 9];
    assert_eq!(max_salary(a), 99641);
}

#[test]
fn case_3() {
    let a = vec![23, 39, 92];
    assert_eq!(max_salary(a), 923923);
}
