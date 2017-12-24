fn main() {
    println!("{}", sum_factorial(100.0));
}

fn factorial(n: f64) -> f64 {
    if n == 1.0 {
        return 1.0;
    }
    n * factorial(n - 1.0)
}

fn sum_factorial(n: f64) -> usize {
    let f = factorial(n);
    let num_str = format!("{:.*}", 0, f);

    num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_3() {
        assert_eq!(factorial(3.0), 3.0 * 2.0 * 1.0);
    }

    #[test]
    fn factorial_10() {
        assert_eq!(factorial(10.0), 3628800.0);
    }

    #[test]
    fn sum_factorial_10() {
        assert_eq!(sum_factorial(10.0), 27);
    }
}
