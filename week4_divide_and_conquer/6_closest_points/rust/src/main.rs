use std::io::stdin;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn dist_to(&self, to: &Self) -> f64 {
        let delta_x = (to.x - self.x) as f64;
        let delta_y = (to.y - self.y) as f64;

        delta_x.hypot(delta_y)
    }
}

fn main() {
    let n: u32 = read_line().trim().parse().unwrap();
    let mut points = read_points(n, read_line);

    let result = find_min_dist(&mut points);
    println!("{:.4}", result);
}

fn find_min_dist(points: &mut Vec<Point>) -> f64 {
    fn internal(points: &[Point]) -> f64 {
        match points {
            [] | [_] => std::f64::MAX,
            [x, y] => x.dist_to(y),
            _ => {
                let mid = points.len() / 2;
                let midpoint = &points[mid];
                let part1 = &points[..mid];
                let part2 = &points[mid..];

                let d1 = internal(part1);
                let d2 = internal(part2);
                let mut d = d1.min(d2);

                let from = midpoint.x as f64 - d;
                let to = midpoint.x as f64 + d;
                let mut midpoints =
                    part1.iter()
                        .rev()
                        .take_while(|&p| p.x as f64 > from)
                        .chain(
                            part2.iter()
                                .take_while(|&p| p.x as f64 <= to))
                        .map(|&p| p)
                        .collect::<Vec<_>>();
                midpoints.sort_unstable_by_key(|p| p.y);

                for (i, p) in midpoints[..midpoints.len() - 1].iter().enumerate() {
                    d = midpoints.iter()
                        .skip(i + 1)
                        .take(7)
                        .map(|x| x.dist_to(p))
                        .fold(std::f64::MAX, |acc, x| if x < acc { x } else { acc })
                        .min(d);
                }

                d
            }
        }
    }

    points.sort_unstable_by_key(|x| x.x);
    internal(points)
}

fn read_points(n: u32, read_line: fn() -> String) -> Vec<Point> {
    (0..n).map(|_| read_point(read_line)).collect()
}

fn read_point(read_line: fn() -> String) -> Point {
    let input = read_line().trim().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<_>>();
    Point { x: input[0], y: input[1] }
}

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input
}

// #[cfg(test)]
// mod tests;