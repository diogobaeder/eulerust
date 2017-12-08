fn find_collatz_n(n: usize) -> usize {
    let mut quant = 1;
    let mut cur = n;

    while cur != 1 {
        if cur % 2 == 0 {
            cur /= 2;
        } else {
            cur = 3 * cur + 1;
        }
        quant += 1;
    }

    quant
}

fn main() {
    let mut quant = 0;
    let mut n = 0;

    for i in 1..1_000_000 {
        let q = find_collatz_n(i);
        if q > quant {
            quant = q;
            n = i;
        }
    }

    println!("{}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_collatz_1() {
        assert_eq!(find_collatz_n(1), 1);
    }

    #[test]
    fn finds_collatz_2() {
        assert_eq!(find_collatz_n(2), 2);
    }

    #[test]
    fn finds_collatz_4() {
        assert_eq!(find_collatz_n(4), 3);
    }

    #[test]
    fn finds_collatz_16() {
        assert_eq!(find_collatz_n(16), 5);
    }

    #[test]
    fn finds_collatz_5() {
        assert_eq!(find_collatz_n(5), 6);
    }
}
