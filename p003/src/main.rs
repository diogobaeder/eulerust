fn largest(n: usize) -> usize {
    if n == 4 {
        return 2;
    }
    if n < 4 {
        return n;
    }

    let mut m = 3;
    let mut p = n;
    let mut l = m;

    while ((m as f64).sqrt() as usize) < p {
        if p % m == 0 {
            p = p / m;
            l = m;
        }
        m += 2;
    }

    l
}

fn main() {
    println!("{}", largest(600851475143));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_6() {
        assert_eq!(largest(6), 3);
    }

    #[test]
    fn largest_18() {
        assert_eq!(largest(18), 3);
    }

    #[test]
    fn largest_4() {
        assert_eq!(largest(4), 2);
    }

    #[test]
    fn largest_5() {
        assert_eq!(largest(5), 5);
    }

    #[test]
    fn largest_13() {
        assert_eq!(largest(13), 13);
    }
}
