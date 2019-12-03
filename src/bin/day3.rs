use rayon::prelude::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn parse_points(actions: Vec<&str>) -> Vec<Point> {
    let mut points: Vec<Point> = vec![];

    let mut x = 0;
    let mut y = 0;
    for action in actions {
        let parts = action.split_at(1);
        let n = parts.1.parse::<i32>().unwrap();

        match parts.0 {
            "R" => {
                for _ in 0..n {
                    points.push(Point::new(x, y));
                    x += 1;
                }
            }
            "L" => {
                for _ in 0..n {
                    points.push(Point::new(x, y));
                    x -= 1;
                }
            }
            "U" => {
                for _ in 0..n {
                    points.push(Point::new(x, y));
                    y += 1;
                }
            }
            "D" => {
                for _ in 0..n {
                    points.push(Point::new(x, y));
                    y -= 1;
                }
            }
            _ => unreachable!(),
        }
    }

    points
}

fn main() {
    let input = include_str!("../../inputs/day03.txt"); // 159
    let line_points: Vec<Vec<Point>> = input
        .lines()
        .map(|line| line.split(',').collect::<Vec<_>>())
        .map(parse_points)
        .collect::<Vec<Vec<Point>>>();

    let mut result = line_points[0]
        .par_iter()
        .enumerate()
        .filter(|(_, p)| p.x != 0 && p.y != 0)
        .filter_map(|(s0, p0)| {
            if let Some(s1) = line_points[1].iter().position(|p1| p0 == p1) {
                return Some((s0, s1, p0));
            }
            None
        })
        .map(|(s0, s1, p)| {
            let distance = (0 - p.x).abs() + (0 - p.y).abs();
            let steps = s0 + s1;
            println!("{:?}: \tDistance: {} \tSteps: {}", p, distance, steps);
            (steps, distance)
        })
        .collect::<Vec<(usize, i32)>>();
    result.par_sort_by(|a, b| a.0.cmp(&b.0));

    println!("Steps: {} - Distance: {}", result[0].0, result[0].1);
}
