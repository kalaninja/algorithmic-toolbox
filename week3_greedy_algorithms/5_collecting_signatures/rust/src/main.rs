#[derive(Debug, Clone, Copy)]
struct Range { start: u32, end: u32 }

impl Range {
    fn new(start: u32, end: u32) -> Range { Range { start, end } }
}

fn main() {
    let n: usize = read_line().parse().unwrap();
    let ranges =
        (0..n).map(|_| {
            let range: Vec<u32> = read_line().split_whitespace().map(|x| x.parse().unwrap()).collect();
            Range::new(range[0], range[1])
        }).collect::<Vec<_>>();

    let result = find_overlaps(ranges).iter().map(ToString::to_string).collect::<Vec<_>>();
    println!("{}", result.len());
    println!("{}", result.join(" "));
}

fn find_overlaps(mut ranges: Vec<Range>) -> Vec<u32> {
    ranges.sort_unstable_by_key(|x| x.end);

    let first = ranges[0].end;
    let mut result = vec![first];
    let mut curr = first;
    for range in ranges.iter().skip(1) {
        if range.start <= curr { continue; }

        curr = range.end;
        result.push(curr)
    }

    result
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

#[cfg(test)]
mod tests;