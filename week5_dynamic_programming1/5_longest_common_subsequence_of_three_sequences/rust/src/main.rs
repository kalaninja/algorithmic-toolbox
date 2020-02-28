use std::ops::{Index, IndexMut};

struct Cube {
    d: Vec<Vec<Vec<u32>>>,
    n: usize,
    m: usize,
    l: usize,
}

impl Cube {
    fn new(n: usize, m: usize, l: usize) -> Self {
        Cube { d: vec![vec![vec![0; l]; m]; n], n, m, l }
    }

    fn last(&self) -> u32 {
        self[self.n - 1][self.m - 1][self.l - 1]
    }
}

impl Index<usize> for Cube {
    type Output = Vec<Vec<u32>>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.d[index]
    }
}

impl IndexMut<usize> for Cube {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.d[index]
    }
}

fn main() {
    let _n = read_line();
    let a = read_array(read_line);
    let _m = read_line();
    let b = read_array(read_line);
    let _l = read_line();
    let c = read_array(read_line);

    let result = lcs3(a, b, c);
    println!("{}", result);
}

fn lcs3(a: Vec<i32>, b: Vec<i32>, c: Vec<i32>) -> u32 {
    let mut cube = Cube::new(a.len() + 1, b.len() + 1, c.len() + 1);

    for i in 1..cube.n {
        for j in 1..cube.m {
            for k in 1..cube.l {
                if a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1] {
                    cube[i][j][k] = cube[i - 1][j - 1][k - 1] + 1;
                } else {
                    cube[i][j][k] = cube[i - 1][j][k]
                        .max(cube[i][j - 1][k])
                        .max(cube[i][j][k - 1]);
                }
            }
        }
    }

    cube.last()
}

fn read_array(read_line: fn() -> String) -> Vec<i32> {
    read_line()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

// #[cfg(test)]
// mod tests;
