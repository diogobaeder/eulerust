fn sum(n: u32) -> u32 {
    let mut res = 0;

    for i in 0..n {
        if i % 3 == 0 || i % 5 == 0 {
            res += i
        }
    }

    res
}

fn main() {
    println!("{}", sum(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_0() {
        assert_eq!(sum(0), 0);
    }

    #[test]
    fn sums_6() {
        assert_eq!(sum(6), 8);
    }

    #[test]
    fn sums_7() {
        assert_eq!(sum(7), 14);
    }
}
