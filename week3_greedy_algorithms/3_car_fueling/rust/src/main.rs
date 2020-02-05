use std::io;

fn main() {
    let d: u32 = read_line().unwrap().parse().unwrap();
    let m: u32 = read_line().unwrap().parse().unwrap();
    let _n = read_line();

    let stops: Vec<u32> = read_line().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", calc_stops(stops, d, m))
}

fn calc_stops(stops: Vec<u32>, d: u32, m: u32) -> i32 {
    let mut traveled: u32 = 0;
    let mut result = 0;
    for i in 0..stops.len() {
        if d - traveled <= m || stops[i] - traveled > m { break; }

        if i < stops.len() - 1 && stops[i + 1] - traveled <= m {
            continue;
        }

        result += 1;
        traveled = stops[i];
    }

    if d - traveled <= m { result } else { -1 }
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

#[test]
fn case_1() {
    let stops = vec![200, 375, 550, 750];

    assert_eq!(calc_stops(stops, 950, 400), 2);
}

#[test]
fn case_2() {
    let stops = vec![1, 2, 5, 9];

    assert_eq!(calc_stops(stops, 10, 3), -1);
}

#[test]
fn case_3() {
    let stops = vec![3, 5];

    assert_eq!(calc_stops(stops, 6, 3), 1);
}