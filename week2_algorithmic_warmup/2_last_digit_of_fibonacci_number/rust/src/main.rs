use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u32>().unwrap();

    println!("{}", fibonacci_last_digit(n));
}

fn fibonacci_last_digit(n: u32) -> u32 {
    if n > 1 {
        let mut prev = 0;
        let mut curr = 1;

        for _ in 2..=n {
            let next = (curr + prev) % 10;
            prev = curr;
            curr = next;
        }

        curr
    } else { n }
}
