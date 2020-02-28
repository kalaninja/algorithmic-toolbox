use crate::*;

#[test]
fn test_case_1() {
    let a = vec![1, 2, 3];
    let b = vec![2, 1, 3];
    let c = vec![1, 3, 5];
    assert_eq!(lcs3(a, b, c), 2);
}

#[test]
fn test_case_2() {
    let a = vec![8, 3, 2, 1, 7];
    let b = vec![8, 2, 1, 3, 8, 10, 7];
    let c = vec![6, 8, 3, 1, 4, 7];
    assert_eq!(lcs3(a, b, c), 3);
}

#[test]
fn test_case_3() {
    let a = vec![8, 2];
    let b = vec![6, 8];
    let c = vec![8, 2];
    assert_eq!(lcs3(a, b, c), 1);
}
