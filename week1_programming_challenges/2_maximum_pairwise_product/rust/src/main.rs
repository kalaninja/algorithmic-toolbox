fn main() {
    let _n: i32 = read_ln().trim().parse().unwrap();

    let numbers = read_ln().trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();

    println!("{}", get_max_pairwise_product(&numbers));
}

fn get_max_pairwise_product(numbers: &[i32]) -> i64 {
    let pair: (i32, i32) = numbers.iter().fold((0, 0), |mut acc, &x| {
        if x > acc.0 {
            acc.1 = acc.0;
            acc.0 = x;
        } else if x > acc.1 {
            acc.1 = x;
        }

        acc
    });

    pair.0 as i64 * pair.1 as i64
}

#[test]
fn test_get_max_pairwise_product() {
    let numbers = [1, 100000, 3, 90000];
    let result = get_max_pairwise_product(&numbers);

    assert_eq!(result, 9000000000)
}

fn read_ln() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line
}