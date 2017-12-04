fn largest(n: u32) -> usize {
    let highest_num = (10 as usize).pow(n) - 1;
    let lowest_num = (10 as usize).pow(n - 1);

    let mut left = highest_num;
    let mut l = 0;

    while left >= lowest_num {
        let mut right = left;
        while right >= lowest_num {
            let mult = left * right;
            if mult == reverse(mult) && (l == 0 || mult > l) {
                l = mult;
            }
            right -= 1;
        }
        left -= 1;
    }
    l
}

fn reverse(n: usize) -> usize {
    let mut m = n;
    let mut res = 0;

    while m > 0 {
        res = (res * 10) + (m % 10);
        m /= 10;
    }
    res
}

fn main() {
    println!("{}", largest(3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_for_1_digit() {
        assert_eq!(largest(1), 9);
    }

    #[test]
    fn largest_for_2_digits() {
        assert_eq!(largest(2), 9009);
    }

    #[test]
    fn reverse_of_1() {
        assert_eq!(reverse(1), 1);
    }

    #[test]
    fn reverse_of_12() {
        assert_eq!(reverse(12), 21);
    }

    #[test]
    fn reverse_of_263() {
        assert_eq!(reverse(263), 362);
    }
}
