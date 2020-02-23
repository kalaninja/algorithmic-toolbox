use crate::*;

#[test]
fn test_case_1() {
    let mut points = vec![
        Point { x: 0, y: 0 },
        Point { x: 3, y: 4 },
    ];

    let result = find_min_dist(&mut points);
    assert_eq!(result, 5.0);
}

#[test]
fn test_case_2() {
    let mut points = vec![
        Point { x: 7, y: 7 },
        Point { x: 1, y: 100 },
        Point { x: 4, y: 8 },
        Point { x: 7, y: 7 },
    ];

    let result = find_min_dist(&mut points);
    assert_eq!(result, 0.0);
}

#[test]
fn test_case_3() {
    let mut points = vec![
        Point { x: 4, y: 4 },
        Point { x: -2, y: -2 },
        Point { x: -3, y: -4 },
        Point { x: -1, y: 3 },
        Point { x: 2, y: 3 },
        Point { x: -4, y: 0 },
        Point { x: 1, y: 1 },
        Point { x: -1, y: -1 },
        Point { x: 3, y: -1 },
        Point { x: -4, y: 2 },
        Point { x: -2, y: 4 },
    ];

    let result = find_min_dist(&mut points);
    assert_eq!(result, std::f64::consts::SQRT_2);
}
