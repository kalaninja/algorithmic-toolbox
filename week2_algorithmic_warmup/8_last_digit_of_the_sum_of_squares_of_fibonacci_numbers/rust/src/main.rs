fn main() {
//    println!("{:?}", _get_period());

    let period = [0, 1, 1, 2, 3, 5, 8, 3, 1, 4, 5, 9, 4, 3, 7, 0, 7, 7, 4, 1, 5, 6, 1, 7,
        8, 5, 3, 8, 1, 9, 0, 9, 9, 8, 7, 5, 2, 7, 9, 6, 5, 1, 6, 7, 3, 0, 3, 3, 6, 9, 5, 4, 9, 3,
        2, 5, 7, 2, 9, 1];

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    let result = (period[(n % 60) as usize] * period[((n + 1) % 60) as usize]) % 10;
    println!("{}", result);
}

fn _get_period() -> Vec<u32> {
    let mut period = vec![0];

    let mut curr = 1;
    loop {
        let prev = *period.last().unwrap();
        let next = (curr + prev) % 10;
        if curr == 0 && prev == 1 { break; }

        period.push(curr);
        curr = next;
    }

    period
}