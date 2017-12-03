fn sum(n: u32) -> u32 {
    let mut res = 2;
    let mut left = 1;
    let mut right = 2;
    let mut next = 3;

    while next < n {
        left = right;
        right = next;
        next = left + right;
        if next % 2 == 0 {
            res += next;
        }
    }
    return res;
}

fn main() {
    println!("{}", sum(4_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_5() {
        assert_eq!(sum(5), 2);
    }

    #[test]
    fn sums_8() {
        assert_eq!(sum(8), 10);
    }

    #[test]
    fn sums_13() {
        assert_eq!(sum(13), 10);
    }

    #[test]
    fn sums_55() {
        assert_eq!(sum(55), 44);
    }
}
