fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let result = find_prizes(n).iter().map(ToString::to_string).collect::<Vec<_>>();

    println!("{}", result.len());
    println!("{}", result.join(" "));
}

fn find_prizes(mut n: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    let mut i = 1;
    loop {
        if i * 2 >= n {
            result.push(n);
            break;
        }

        result.push(i);
        n -= i;
        i += 1;
    }

    result
}

#[test]
fn case_1() {
    assert_eq!(find_prizes(6), vec![1, 2, 3])
}

#[test]
fn case_2() {
    assert_eq!(find_prizes(8), vec![1, 2, 5])
}

#[test]
fn case_3() {
    assert_eq!(find_prizes(2), vec![2])
}
