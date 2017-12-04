fn min_mult(n: usize) -> usize {
    let mut numbers: Vec<usize> = (1..n + 1).collect();
    let mut factors = vec![];
    let mut divider = 2;

    loop {
        let mut needs_division = false;
        let mut found = false;

        for number in numbers.iter_mut() {
            if *number == 1 {
                continue;
            }
            needs_division = true;
            if *number % divider == 0 {
                found = true;
                *number /= divider;
            }
        }

        if !needs_division {
            break;
        }

        if found {
            factors.push(divider);
        } else {
            divider += 1;
        }
    }

    factors.iter().product()
}

fn main() {
    println!("{}", min_mult(20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_mult_3() {
        assert_eq!(min_mult(3), 6);
    }

    #[test]
    fn min_mult_4() {
        assert_eq!(min_mult(4), 12);
    }

    #[test]
    fn min_mult_10() {
        assert_eq!(min_mult(10), 2520);
    }
}
