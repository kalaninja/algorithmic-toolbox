fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums = input.trim().split_whitespace().collect::<Vec<_>>();

    println!("{}", fibonacci_huge(nums[0].parse().unwrap(), nums[1].parse().unwrap()));
}

fn fibonacci_huge(n: u64, m: u32) -> u32 {
    if n <= 1 { n as u32 } else {
        let mut period = Vec::new();
        period.push(0);
        period.push(1);

        let mut prev = 0;
        let mut curr = 1;
        let mut i = 2;
        loop {
            let next = (prev + curr) % m;
            if i == n { return next; }

            if next == 1 && curr == 0 { return period[(n % (period.len() as u64 - 1)) as usize]; }

            period.push(next);
            prev = curr;
            curr = next;
            i += 1;
        }
    }
}

#[test]
fn test_fibonacci_huge() {
    assert_eq!(fibonacci_huge(7, 10), 3)
}