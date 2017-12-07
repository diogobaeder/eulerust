struct TriangularIterator {
    i: usize,
    sum: usize,
}

impl TriangularIterator {
    fn new() -> TriangularIterator {
        TriangularIterator { i: 0, sum: 0 }
    }
}

impl Iterator for TriangularIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        self.sum += self.i;

        let mut dividers = 0;

        for n in 1..(((self.sum as f64).sqrt() as usize) + 1) {
            if self.sum % n == 0 {
                dividers += 1;
                if n != self.sum / n {
                    dividers += 1;
                }
            }
        }

        Some((self.sum, dividers))
    }
}

fn main() {
    let iterator = TriangularIterator::new();

    for (sum, dividers) in iterator {
        if dividers > 500 {
            println!("{}", sum);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangular_iterator() {
        let mut iterator = TriangularIterator::new();

        assert_eq!(iterator.next(), Some((1, 1)));
        assert_eq!(iterator.next(), Some((3, 2)));
        assert_eq!(iterator.next(), Some((6, 4)));
        assert_eq!(iterator.next(), Some((10, 4)));
        assert_eq!(iterator.next(), Some((15, 4)));
        assert_eq!(iterator.next(), Some((21, 4)));
        assert_eq!(iterator.next(), Some((28, 6)));
    }
}
