fn primes(n: usize) -> Vec<usize> {
    let mut flags = vec![true; n];
    let sqrt = (n as f64).sqrt() as usize;
    for i in 2..(sqrt + 1) {
        if flags[i] {
            let mut next = i.pow(2);
            while next < n {
                flags[next] = false;
                next += i;
            }
        }
    }

    let mut primes = Vec::new();
    for (i, is_prime) in flags.iter().enumerate() {
        if *is_prime {
            primes.push(i);
        }
    }

    primes[2..].to_vec()
}



fn largest(n: usize) -> usize {
    let p = primes(n + 1);

    if &n == &p[p.len() - 1] {
        return n;
    }

    let mut largest = 2;

    for prime in p {
        if prime > n / 2 {
            break;
        }
        if n % prime == 0 {
            largest = prime;
        }
    }

    largest
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
