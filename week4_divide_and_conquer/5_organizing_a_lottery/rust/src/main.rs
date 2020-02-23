use crate::PointKind::*;
use std::{str::FromStr, fmt::Debug};

struct Segment {
    start: i32,
    end: i32,
}

impl Segment {
    fn from_vec(vec: Vec<i32>) -> Self { Self { start: vec[0], end: vec[1] } }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Point {
    x: i32,
    kind: PointKind,
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum PointKind {
    SegmentStart,
    Static { index: usize },
    SegmentEnd,
}

impl Point {
    fn new(x: i32, kind: PointKind) -> Self {
        Point { x, kind }
    }
}

fn main() {
    let line1 = read_line();
    let segment_n = line1[0];
    let static_n = line1[1];

    let mut points = get_points(segment_n, static_n, read_line);
    let result = solve(&mut points, static_n);

    println!("{}", result.iter().map(ToString::to_string).collect::<Vec<_>>().join(" "))
}

fn solve(points: &mut Vec<Point>, static_n: usize) -> Vec<u32> {
    points.sort_unstable();

    let mut result = vec![Default::default(); static_n];
    let mut segment_count = 0;
    for point in points {
        match point {
            Point { x: _, kind: SegmentStart } => segment_count += 1,
            Point { x: _, kind: SegmentEnd } => segment_count -= 1,
            Point { x: _, kind: Static { index } } => result[*index] = segment_count,
        }
    }

    result
}

#[test]
fn test_case_1() {
    let mut points = vec![
        Point::new(0, SegmentStart),
        Point::new(5, SegmentEnd),
        Point::new(7, SegmentStart),
        Point::new(10, SegmentEnd),
        Point::new(1, Static { index: 0 }),
        Point::new(6, Static { index: 1 }),
        Point::new(11, Static { index: 2 })
    ];

    let result = solve(&mut points, 3);
    assert_eq!(result, vec![1, 0, 0])
}

#[test]
fn test_case_2() {
    let mut points = vec![
        Point::new(-10, SegmentStart),
        Point::new(10, SegmentEnd),
        Point::new(-100, Static { index: 0 }),
        Point::new(100, Static { index: 1 }),
        Point::new(0, Static { index: 2 })
    ];

    let result = solve(&mut points, 3);
    assert_eq!(result, vec![0, 0, 1])
}

#[test]
fn test_case_3() {
    let mut points = vec![
        Point::new(0, SegmentStart),
        Point::new(5, SegmentEnd),
        Point::new(-3, SegmentStart),
        Point::new(2, SegmentEnd),
        Point::new(7, SegmentStart),
        Point::new(10, SegmentEnd),
        Point::new(1, Static { index: 0 }),
        Point::new(6, Static { index: 1 }),
    ];

    let result = solve(&mut points, 2);
    assert_eq!(result, vec![2, 0])
}

#[test]
fn test_case_4() {
    let mut points = vec![
        Point::new(-10, SegmentStart),
        Point::new(5, SegmentEnd),
        Point::new(0, SegmentStart),
        Point::new(5, SegmentEnd),
        Point::new(5, Static { index: 0 }),
    ];

    let result = solve(&mut points, 1);
    assert_eq!(result, vec![2])
}

fn get_points(segment_n: usize, static_n: usize, read_line: fn() -> Vec<i32>) -> Vec<Point> {
    let mut result = Vec::with_capacity(segment_n * 2 + static_n);

    for _ in 0..segment_n {
        let segment = get_segment(read_line);
        result.push(Point::new(segment.start, SegmentStart));
        result.push(Point::new(segment.end, SegmentEnd));
    }

    result.extend(
        read_line()
            .iter()
            .enumerate()
            .map(|(index, &x)| Point::new(x, Static { index })));

    result
}

fn get_segment(read_line: fn() -> Vec<i32>) -> Segment {
    Segment::from_vec(read_line())
}

fn read_line<T>() -> Vec<T> where T: FromStr, <T as FromStr>::Err: Debug {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect()
}
