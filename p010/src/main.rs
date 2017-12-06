fn prime_sum(n: usize) -> usize {
    let mut primes: Vec<usize> = Vec::with_capacity(n);
    let mut cur_num: usize = 3;
    let mut cur_sum: usize = 2;

    while cur_num < n {
        if !primes.iter().any(|x| cur_num % x == 0) {
            primes.push(cur_num);
            cur_sum += cur_num;
        }
        cur_num += 2;
    }
    cur_sum
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
