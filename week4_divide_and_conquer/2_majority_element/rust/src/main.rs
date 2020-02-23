use std::io::stdin;

fn main() {
    read_line();
    let vec = read_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let result = if has_majority(vec) { 1 } else { 0 };

    println!("{}", result);
}

fn has_majority(mut vec: Vec<u32>) -> bool {
    let majority = vec.len() / 2 + 1;
    let mut left = 0;
    let mut right = vec.len() - 1;

    loop {
        let (lt, gt) = partition_3(&mut vec, left, right);

        if gt - lt + 1 >= majority {
            return true;
        }

        if lt - left + 1 >= majority {
            right = lt - 1;
        } else if right - gt + 1 >= majority {
            left = gt + 1;
        } else {
            return false;
        }
    }
}

fn partition_3(vec: &mut Vec<u32>, left: usize, right: usize) -> (usize, usize) {
    let pivot = vec[left];
    let mut lt = left;
    let mut gt = right;

    let mut i = left + 1;
    while i <= gt {
        if vec[i] < pivot {
            vec.swap(i, lt);
            lt += 1;
            i += 1;
        } else if vec[i] > pivot {
            vec.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }

    (lt, gt)
}

#[test]
fn test_case1() {
    assert_eq!(has_majority(vec![2, 3, 9, 2, 2]), true)
}

#[test]
fn test_case2() {
    assert_eq!(has_majority(vec![1, 2, 3, 4]), false)
}

#[test]
fn test_case_3() {
    let vec = vec![512766168, 717383758, 5, 126144732, 5, 573799007, 5, 5, 5, 405079772];
    assert_eq!(has_majority(vec), false)
}

#[test]
fn test_case3() {
    assert_eq!(has_majority(vec![1, 2, 3, 1]), false)
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}