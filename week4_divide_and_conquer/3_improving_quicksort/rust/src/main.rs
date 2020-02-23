fn main() {
    let _ = read_line();
    let mut vec =
        read_line()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

    quick_sort(&mut vec);
    let result = vec.iter().map(ToString::to_string).collect::<Vec<_>>().join(" ");
    println!("{}", result);
}

fn quick_sort(vec: &mut Vec<u32>) {
    fn quick_sort_internal(vec: &mut Vec<u32>, l: usize, mut r: usize) {
        while l < r {
            let (lt, gt) = partition_3(vec, l, r);
            quick_sort_internal(vec, gt + 1, r);

            if lt > 0 { r = lt - 1; } else { return; }
        }
    }

    quick_sort_internal(vec, 0, vec.len() - 1);
}

fn partition_3(vec: &mut Vec<u32>, l: usize, r: usize) -> (usize, usize) {
    let pivot = vec[l];

    let mut lt = l;
    let mut gt = r;

    let mut i = l + 1;
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
fn test() {
    let mut tests = [
        (vec![2, 3, 9, 2, 2], vec![2u32, 2, 2, 3, 9]),
        (vec![1], vec![1]),
        (vec![2, 1], vec![1, 2]),
        (vec![1, 1, 1], vec![1, 1, 1])
    ];

    tests.iter_mut().for_each(|(d, a)| {
        quick_sort(d);
        assert_eq!(d, a)
    });
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}
