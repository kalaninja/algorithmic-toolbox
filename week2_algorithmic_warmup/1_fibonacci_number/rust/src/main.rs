use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    let n = line.trim().parse::<u32>().unwrap();
    println!("{}", fibonacci_list(n));
}

fn _fibonacci_naive(n: u32) -> u32 {
    if n > 1 { _fibonacci_naive(n - 1) + _fibonacci_naive(n - 2) } else { n }
}

fn fibonacci_list(n: u32) -> u32 {
    if n > 1 {
        let mut f = Vec::with_capacity((n + 1) as usize);
        f.push(0);
        f.push(1);

        for i in 2..=n as usize {
            let a = f[i - 1];
            let b = f[i - 2];
            f.push(a + b);
        }

        *f.last().unwrap()
    } else { n }
}

#[test]
fn test_same_result() {
    for n in 0..=45 {
        assert_eq!(_fibonacci_naive(n), fibonacci_list(n))
    }
}
