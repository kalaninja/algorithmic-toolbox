fn main() {
//    println!("{:?}", _get_sum_period());

    let sum_period = [0, 1, 2, 4, 7, 2, 0, 3, 4, 8, 3, 2, 6, 9, 6, 6, 3, 0, 4, 5, 0, 6, 7,
        4, 2, 7, 0, 8, 9, 8, 8, 7, 6, 4, 1, 6, 8, 5, 4, 0, 5, 6, 2, 9, 2, 2, 5, 8, 4, 3, 8, 2, 1,
        4, 6, 1, 8, 0, 9, 0];

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let numbers = input.trim()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let (m, n) = (numbers[0], numbers[1]);

    let n_sum = sum_period[(n % 60) as usize];
    let result = if m == 0 {
        n_sum
    } else {
        let m_red = sum_period[((m - 1) % 60) as usize];
        if n_sum > m_red { n_sum - m_red } else { n_sum + 10 - m_red }
    };
    println!("{}", result);
}

fn _get_sum_period() -> Vec<u32> {
    let mut period = vec![(0, 0)];

    let mut curr = 1;
    loop {
        let prev = period.last().unwrap().clone();
        let next = (curr + prev.0) % 10;

        if next == 1 && curr == 0 { return period.iter().map(|(_, s)| *s).collect(); }

        period.push((curr, (prev.1 + curr) % 10));
        curr = next;
    }
}
