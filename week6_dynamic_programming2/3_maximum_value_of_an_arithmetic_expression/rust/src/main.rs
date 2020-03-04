// #[derive(Debug, PartialEq)]
enum Op {
    Plus,
    Minus,
    Multi,
}

impl Op {
    fn apply(&self, x: i64, y: i64) -> i64 {
        match self {
            Op::Plus => { x + y }
            Op::Minus => { x - y }
            Op::Multi => { x * y }
        }
    }

    fn from_char(ch: char) -> Self {
        match ch {
            '+' => Op::Plus,
            '-' => Op::Minus,
            '*' => Op::Multi,
            _ => unimplemented!()
        }
    }
}

fn main() {
    let expr = read_line();
    let (nums, ops) = parse_expr(expr);

    let result = max_value(nums, ops);
    println!("{}", result);
}

fn max_value(nums: Vec<u32>, ops: Vec<Op>) -> i64 {
    let n = nums.len();
    let mut m = vec![vec![0; n]; n];

    nums.iter().enumerate().for_each(|(i, x)| m[i][i] = *x as i64);

    for j in 1..n {
        for i in 0..n - j {
            let from = i;
            let to = j + i;

            let mut min = std::i64::MAX;
            let mut max = std::i64::MIN;

            for k in from..to {
                let a = ops[k].apply(m[from][k], m[k + 1][to]);
                let b = ops[k].apply(m[from][k], m[to][k + 1]);
                let c = ops[k].apply(m[k][from], m[k + 1][to]);
                let d = ops[k].apply(m[k][from], m[to][k + 1]);

                min = min.min(a).min(b).min(c).min(d);
                max = max.max(a).max(b).max(c).max(d);
            }

            m[from][to] = max;
            m[to][from] = min;
        }
    }

    m[0][n - 1]
}

fn parse_expr(expr: String) -> (Vec<u32>, Vec<Op>) {
    let n = expr.len() / 2;

    expr
        .trim()
        .char_indices()
        .fold((Vec::with_capacity(n), Vec::with_capacity(n - 1)),
              |mut acc, (i, ch)| {
                  if i % 2 == 0 {
                      acc.0.push(ch.to_digit(10).unwrap())
                  } else {
                      acc.1.push(Op::from_char(ch))
                  }

                  acc
              })
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

// #[cfg(test)]
// mod tests;