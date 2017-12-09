fn paths(n: usize) -> usize {
    let mut matrix = vec![vec![0; n]; n];

    matrix[0][0] = 2;

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            match (i, j) {
                (0, 0) => matrix[i][j] = 2,
                (0, j) => matrix[i][j] = matrix[i][j - 1] + 1,
                (i, 0) => matrix[i][j] = matrix[i - 1][j] + 1,
                (i, j) => matrix[i][j] = matrix[i - 1][j] + matrix[i][j - 1],
            }
        }
    }

    matrix[n - 1][n - 1]
}

fn main() {
    println!("{}", paths(20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_1() {
        assert_eq!(paths(1), 2);
    }

    #[test]
    fn paths_2() {
        assert_eq!(paths(2), 6);
    }

    #[test]
    fn paths_3() {
        assert_eq!(paths(3), 20);
    }
}
