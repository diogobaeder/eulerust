fn next_path(i: u8, j: u8, n: u8, q: usize) -> usize {
    if i == n && j == n {
        return q + 1;
    }

    let mut q = q;

    if i < n {
        q = next_path(i + 1, j, n, q);
    }

    if j < n {
        q = next_path(i, j + 1, n, q);
    }

    q
}

fn paths(n: u8) -> usize {
    return next_path(0, 0, n, 0);
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
}
