use crate::*;

#[test]
fn test_parse_expr() {
    let tests = vec![
        ("1+5", (vec![1_u32, 5], vec![Op::Plus])),
        ("5-8+7*4-8+9", (vec![5, 8, 7, 4, 8, 9], vec![Op::Minus, Op::Plus, Op::Multi, Op::Minus, Op::Plus]))
    ];

    tests.iter().for_each(|(d, e)| {
        assert_eq!(parse_expr(d.to_string()), *e);
    })
}

#[test]
fn test_case_1() {
    assert_eq!(max_value(vec![1_u32, 5], vec![Op::Plus]), 6);
}

#[test]
fn test_case_2() {
    let nums = vec![5, 8, 7, 4, 8, 9];
    let ops = vec![Op::Minus, Op::Plus, Op::Multi, Op::Minus, Op::Plus];
    assert_eq!(max_value(nums, ops), 200);
}
