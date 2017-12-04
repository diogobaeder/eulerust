fn primary(n: usize) -> usize {
    let mut primes: Vec<usize> = Vec::with_capacity(n);
    let mut count = 1;
    let mut next = 2;

    while count <= n {
        let mut found = false;
        for num in primes.iter() {
            if num > &(next / 2 as usize) {
                break;
            }
            if next % num == 0 {
                found = true;
                break;
            }
        }
        if !found {
            primes.push(next);
            count += 1;
        }
        next += 1;
    }

    *(primes.last().unwrap())
}

fn main() {
    println!("{}", primary(10_001));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_1() {
        assert_eq!(primary(1), 2);
    }

    #[test]
    fn primary_2() {
        assert_eq!(primary(2), 3);
    }

    #[test]
    fn primary_3() {
        assert_eq!(primary(3), 5);
    }

    #[test]
    fn primary_6() {
        assert_eq!(primary(6), 13);
    }
}
