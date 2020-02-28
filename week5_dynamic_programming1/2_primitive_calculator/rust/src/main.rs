fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    let (k, a) = solve(n);
    println!("{}", k);
    println!("{}", a.iter().map(ToString::to_string).collect::<Vec<_>>().join(" "))
}

fn solve(n: usize) -> (u32, Vec<u32>) {
    if n == 1 { return (0, vec![1]); }

    let mut s = Vec::with_capacity(n);
    s.push((0, 0));
    for i in 1..n {
        let mut min = {
            let prev = i - 1;
            (s[prev].0, prev)
        };
        if i % 2 == 1 {
            let div2 = i / 2;
            let item = s[div2];
            if item.0 < min.0 { min = (item.0, div2) }
        }
        if i % 3 == 2 {
            let div3 = i / 3;
            let item = s[div3];
            if item.0 < min.0 { min = (item.0, div3) }
        }

        min.0 += 1;
        s.push(min)
    }
    let last = n - 1;
    let mut path = Vec::new();
    let mut cur = last;
    while s[cur].0 > 0 {
        path.push(cur as u32 + 1);
        cur = s[cur].1;
    }
    path.push(1);

    path.reverse();
    (s[last].0, path)
}

#[test]
fn test_case_1() {
    assert_eq!(solve(1), (0, vec![1]))
}

#[test]
fn test_case_2() {
    assert_eq!(solve(5), (3, vec![1, 3, 4, 5]))
}

#[test]
fn test_case_3() {
    let expected = vec![1, 3, 9, 10, 11, 33, 99, 297, 891, 2673, 8019, 16038, 16039, 48117, 96234];
    assert_eq!(solve(96234), (14, expected))
}
