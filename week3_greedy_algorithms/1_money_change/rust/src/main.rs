fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    println!("{}", change(n));
}

fn change(n: u32) -> u32 {
    let mut sum = n;
    let mut result = 0;

    for i in [10, 5, 1].iter() {
        let denom = *i;
        let coin_num = sum / denom;
        sum -= coin_num * denom;

        result += coin_num;
    }

    result
}
