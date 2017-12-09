fn sum(n: u32) -> f64 {
    let pow = (2 as f64).powf(n as f64);

    format!("{}", pow)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as f64)
        .sum()
}

fn main() {
    println!("{}", sum(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_2() {
        assert_eq!(sum(2), 4);
    }

    #[test]
    fn sum_3() {
        assert_eq!(sum(3), 8);
    }

    #[test]
    fn sum_4() {
        assert_eq!(sum(4), 7);
    }
}
