use super::*;
use rand::prelude::*;

#[test]
fn case_1() {
    let range = vec![Range::new(1, 3), Range::new(2, 5), Range::new(3, 6)];

    assert_eq!(find_overlaps(range), vec![3])
}

#[test]
fn case_2() {
    let range =
        vec![Range::new(4, 7), Range::new(1, 3), Range::new(2, 5), Range::new(5, 6)];

    assert_eq!(find_overlaps(range), vec![3, 6])
}

#[test]
fn case_3() {
    let range =
        vec![Range::new(0, 10), Range::new(0, 10), Range::new(0, 10)];

    assert_eq!(find_overlaps(range), vec![10])
}

#[test]
fn test_random_single() {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1, 101);
    let mut ranges = Vec::with_capacity(n);

    for _ in 0..n {
        let start = rng.gen_range(0, 1000/*000000*/);
        let end = rng.gen_range(start, 1001/*000000*/);
        ranges.push(Range::new(start, end))
    }

    ranges.sort_unstable_by_key(|x| x.end);
    let points = find_overlaps(ranges.to_owned());
    let result = ranges.iter().all(|r| { points.iter().any(|&x| { r.start <= x && x <= r.end }) });
    if !result {
        println!("{:?}", points);
        println!("{:?}", ranges);
    }
    assert_eq!(result, true)
}

#[test]
fn test_random_multi() {
    for _ in 0..10000 {
        test_random_single()
    }
}


