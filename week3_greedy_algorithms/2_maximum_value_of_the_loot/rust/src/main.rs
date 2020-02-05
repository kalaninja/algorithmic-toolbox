struct Cond { n: u32, w: u32 }

#[derive(Debug)]
struct Item { v: u32, w: u32, v_w: f64 }

impl Item {
    fn new(v: u32, w: u32) -> Item {
        Item { v, w, v_w: v as f64 / w as f64 }
    }
}

fn main() {
    let cond =
        {
            let line = read_line();
            Cond { n: line.0, w: line.1 }
        };

    let items =
        (0..cond.n).map(|_| {
            let (v, w) = read_line();
            Item::new(v, w)
        }).collect();

    println!("{:.4}", calc(items, cond.w))
}

fn calc(mut items: Vec<Item>, mut w: u32) -> f64 {
    items.sort_unstable_by(|b, a| a.v_w.partial_cmp(&b.v_w).unwrap());

    let mut result = 0f64;
    for item in items {
        let put = if w >= item.w { item.w } else { w };
        w -= put;
        result += item.v_w * put as f64;
    }

    result
}

fn read_line() -> (u32, u32) {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    (nums[0], nums[1])
}

#[test]
fn case_1() {
    let items = vec![Item::new(60, 20), Item::new(100, 50), Item::new(120, 30)];
    assert_eq!((180.0 - calc(items, 50)).abs() <= 0.0001, true)
}

#[test]
fn case_2() {
    let items = vec![Item::new(500, 30)];
    assert_eq!((166.6667 - calc(items, 10)).abs() <= 0.0001, true)
}