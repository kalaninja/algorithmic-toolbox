fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums = input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    println!("{}", gcd(nums[0], nums[1]));
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else {
        gcd(b, a % b)
    }
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(6, 8), 2)
}