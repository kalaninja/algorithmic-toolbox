extern crate rand;

use crate::*;
use rand::prelude::*;

#[test]
fn test_case_1() {
    let vec = vec![1, 5, 8, 12, 13];
    let search = [8u32, 1, 23, 1, 11];
    let expected = [Some(2), Some(0), None, Some(0), None];

    search.iter()
        .zip(expected.iter())
        .for_each(|(&s, &e)| assert_eq!(binary_search(&vec, s), e));
}

#[test]
fn test_case_2() {
    let vec = vec![1, 2];
    assert_eq!(binary_search(&vec, 2), Some(1));
}

#[test]
fn test_case_3() {
    let vec = vec![18, 32];
    assert_eq!(binary_search(&vec, 17), None);
}

#[test]
fn test_case_4() {
    let vec = vec![18, 32];
    assert_eq!(binary_search(&vec, 33), None);
}

#[test]
fn test_random_single() {
    let n_max = 30/*001*/;
    let a_max = 1000/*000001*/;

    let mut rng = rand::thread_rng();

    let n = rng.gen_range(1, n_max);
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        let mut a = rng.gen_range(1, a_max);
        while vec.contains(&a) {
            a = rng.gen_range(1, a_max);
        }
        vec.push(a);
    }
    vec.sort();

    let elem1 = vec[rng.gen_range(0, n)];
    let actual = binary_search(&vec, elem1);
    let expected = naive_search(&vec, elem1);
    if actual != expected { println!("{} => {:?}", elem1, vec) }
    assert_eq!(actual, expected);

    let elem2 = rng.gen_range(1, a_max);
    let actual = binary_search(&vec, elem2);
    let expected = naive_search(&vec, elem2);
    if actual != expected { println!("{} !=> {:?}", elem2, vec) }
    assert_eq!(actual, expected);
}

#[test]
fn test_random_multi() {
    for _ in 0..1000000 {
        test_random_single()
    }
}

fn naive_search(vec: &Vec<u32>, element: u32) -> Option<usize> {
    for (i, &x) in vec.iter().enumerate() {
        if x == element { return Some(i); }
    }

    None
}
