fn main() {
    let vec: Vec<u32> = read_line()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();

    let result = read_line()
        .split_whitespace()
        .skip(1)
        .map(|x| {
            match binary_search(&vec, x.parse().unwrap()) {
                Some(x) => x.to_string(),
                None => "-1".to_string()
            }
        })
        .collect::<Vec<_>>();

    println!("{}", result.join(" "));
}

fn binary_search(vec: &Vec<u32>, element: u32) -> Option<usize> {
    let mut start = 0usize;
    let mut end = vec.len() - 1;

    while end >= start {
        let mid = (end + start) / 2;
        let mid_element = vec[mid];
        if mid_element == element {
            return Some(mid);
        } else if element > mid_element {
            start = mid + 1;
        } else if mid == 0 {
            return None;
        } else {
            end = mid - 1;
        }
    }

    None
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// #[cfg(test)]
// mod tests;
