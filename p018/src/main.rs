use std::fs::File;
use std::io::prelude::*;

fn read_triangle(path: &str) -> Vec<Vec<usize>> {
    let mut triangle: Vec<Vec<usize>> = Vec::new();

    let mut f = File::open(path).unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        triangle.push(
            line.split_whitespace()
                .map(|num_str| {
                    num_str.trim_left_matches("0").parse::<usize>().unwrap()
                })
                .collect(),
        );
    }

    triangle
}

fn highest_sum(triangle: Vec<Vec<usize>>) -> usize {
    let mut i = triangle.len() - 2;
    let mut prev_row = triangle[i + 1].clone();

    loop {
        let row = triangle[i].clone();
        prev_row = row.iter()
            .enumerate()
            .map(|(j, num)| num + prev_row[j].max(prev_row[j + 1]))
            .collect();
        if i == 0 {
            break;
        }
        i -= 1;
    }

    prev_row[0]
}

fn main() {
    println!("{}", highest_sum(read_triangle("src/triangle.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_triangle() {
        let triangle = read_triangle("src/triangle.txt");

        assert_eq!(triangle[0][0], 75);
        assert_eq!(triangle[14][0], 4);
        assert_eq!(triangle[14][14], 23);
    }

    #[test]
    fn highest_sum_1() {
        let triangle: Vec<Vec<usize>> = vec![vec![1], vec![1, 1], vec![1, 1, 1]];

        assert_eq!(highest_sum(triangle), 3);
    }

    #[test]
    fn highest_sum_2() {
        let triangle: Vec<Vec<usize>> = vec![vec![2], vec![3, 1], vec![1, 4, 1]];

        assert_eq!(highest_sum(triangle), 9);
    }

    #[test]
    fn highest_sum_3() {
        let triangle: Vec<Vec<usize>> = vec![vec![3], vec![1, 4], vec![1, 1, 5]];

        assert_eq!(highest_sum(triangle), 12);
    }
}
