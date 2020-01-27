fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums = input.trim().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

    println!("{}", lcm(nums[0], nums[1]))
}

fn lcm(a: u32, b: u32) -> u64 {
    let mut n = a;
    let mut m = b;
    while m != 0 {
        let buff = n;
        n = m;
        m = buff % n;
    }

    a as u64 * b as u64 / n as u64
}

#[test]
fn test_lcm() {
    assert_eq!(lcm(714552, 374513), 170777928)
}
