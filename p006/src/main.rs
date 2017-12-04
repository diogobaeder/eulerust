fn differ(n: usize) -> usize {
    let numbers = 1..(n + 1);
    let sum_of_squares: usize = numbers.clone().map(|x| x.pow(2)).sum();
    let square_of_sum = numbers.sum::<usize>().pow(2);

    square_of_sum - sum_of_squares
}

fn main() {
    println!("{}", differ(100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn differs_2() {
        assert_eq!(differ(2), 4);
    }

    #[test]
    fn differs_3() {
        assert_eq!(differ(3), 22);
    }

    #[test]
    fn differs_4() {
        assert_eq!(differ(4), 70);
    }

    #[test]
    fn differs_10() {
        assert_eq!(differ(10), 2640);
    }
}
