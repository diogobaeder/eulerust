fn prime_sum(n: usize) -> usize {
    let mut flags: Vec<bool> = vec![true; n - 1];
    let sqrt = (n as f64).sqrt() as usize;

    for i in 2..(sqrt + 1) {
        if *flags.get(i - 2).unwrap() {
            let mut j = i.pow(2);
            while j < n + 1 {
                flags[(j - 2)] = false;
                j += i;
            }
        }
    }
    flags
        .iter()
        .enumerate()
        .map(|(i, f)| if *f { i + 2 } else { 0 })
        .sum::<usize>()
}

fn main() {
    println!("{}", prime_sum(2_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_sum_4() {
        assert_eq!(prime_sum(4), 5);
    }

    #[test]
    fn prime_sum_6() {
        assert_eq!(prime_sum(6), 10);
    }

    #[test]
    fn prime_sum_10() {
        assert_eq!(prime_sum(10), 17);
    }
}
