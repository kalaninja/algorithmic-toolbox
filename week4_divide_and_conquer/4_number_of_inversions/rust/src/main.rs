fn main() {
    let _ = read_line();
    let mut vec = read_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let (_, result) = merge_sort(&mut vec[..]);
    println!("{}", result);
}

fn merge_sort(a: &mut [u32]) -> (Vec<u32>, u32) {
    if a.len() == 1 { return (Vec::from(a), 0); }

    let mid = (a.len() - 1) / 2;
    let (b, b_inv) = merge_sort(&mut a[..=mid]);
    let (c, c_inv) = merge_sort(&mut a[mid + 1..]);

    let mut result = merge(&b, &c);
    result.1 += b_inv + c_inv;
    result
}

fn merge(b: &[u32], c: &[u32]) -> (Vec<u32>, u32) {
    let b_len = b.len();
    let c_len = c.len();

    let mut inversions = 0;
    let mut result = Vec::with_capacity(b_len + c_len);

    let mut b_ptr = 0;
    let mut c_ptr = 0;
    while b_ptr < b_len && c_ptr < c_len {
        if b[b_ptr] > c[c_ptr] {
            result.push(c[c_ptr]);
            c_ptr += 1;
            inversions += (b_len - b_ptr) as u32;
        } else {
            result.push(b[b_ptr]);
            b_ptr += 1;
        }
    }

    let remain = if b_ptr < b_len { &b[b_ptr..] } else { &c[c_ptr..] };
    result.extend_from_slice(remain);

    (result, inversions)
}

#[test]
fn test_merge_sort() {
    let mut tests = [
        (vec![2, 9], (vec![2, 9], 0)),
        (vec![9, 2, 9], (vec![2, 9, 9], 1)),
        (vec![2, 3, 9, 2, 9], (vec![2, 2, 3, 9, 9], 2))
    ];

    tests.iter_mut().for_each(|(d, e)| {
        let actual = merge_sort(d);
        assert_eq!(actual, *e);
    });
}

#[test]
fn test_merge() {
    let result = merge(&[9], &[2, 9]);
    assert_eq!(result, (vec![2, 9, 9], 1))
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}