use std::fs::File;
use std::io::prelude::*;

fn read_triangle(path: &str) -> Vec<Vec<u8>> {
    let mut triangle: Vec<Vec<u8>> = Vec::new();

    let mut f = File::open(path).unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        triangle.push(
            line.split_whitespace()
                .map(|num_str| {
                    num_str.trim_left_matches("0").parse::<u8>().unwrap()
                })
                .collect(),
        );
    }

    triangle
}

fn node_sum(triangle: &[&[u8]], sum: usize, i: usize, j: usize) -> usize {
    let sum = sum + (triangle[i][j] as usize);
    if i == triangle.len() - 1 {
        return sum;
    }
    node_sum(triangle, sum, i + 1, j).max(node_sum(triangle, sum, i + 1, j + 1))
}

fn highest_sum(triangle: Vec<Vec<u8>>) -> usize {
    let row_slices: Vec<&[u8]> = triangle.iter().map(|r| r.as_slice()).collect();
    let triangle_slices: &[&[u8]] = row_slices.as_slice();

    node_sum(triangle_slices, 0, 0, 0)
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
        let triangle: Vec<Vec<u8>> = vec![vec![1], vec![1, 1], vec![1, 1, 1]];

        assert_eq!(highest_sum(triangle), 3);
    }

    #[test]
    fn highest_sum_2() {
        let triangle: Vec<Vec<u8>> = vec![vec![2], vec![3, 1], vec![1, 4, 1]];

        assert_eq!(highest_sum(triangle), 9);
    }

    #[test]
    fn highest_sum_3() {
        let triangle: Vec<Vec<u8>> = vec![vec![3], vec![1, 4], vec![1, 1, 5]];

        assert_eq!(highest_sum(triangle), 12);
    }
}
