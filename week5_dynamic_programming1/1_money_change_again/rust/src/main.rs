fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let money = input.trim().parse().unwrap();
    println!("{}", change(money));
}

fn change(money: usize) -> u32 {
    let mut s = Vec::with_capacity(money);
    s.push(1);
    s.push(2);
    s.push(1);
    s.push(1);

    if money <= 4 { return s[money - 1]; }

    for i in 4..money {
        s.push(1 + s[i - 1].min(s[i - 3]).min(s[i - 4]));
    }

    *s.last().unwrap()
}

#[test]
fn test_case_1() {
    assert_eq!(change(2), 2)
}

#[test]
fn test_case_2() {
    assert_eq!(change(34), 9)
}
