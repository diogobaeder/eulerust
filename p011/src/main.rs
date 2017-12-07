const SAMPLE: &str = r#"
08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
"#;

fn table_to_matrix(table: &str) -> Vec<Vec<usize>> {
    table
        .trim()
        .lines()
        .map(|s| {
            s.trim()
                .split_whitespace()
                .map(|num| {
                    let trimmed = num.trim_left_matches("0");
                    if trimmed == "" {
                        0
                    } else {
                        trimmed.parse::<usize>().unwrap()
                    }
                })
                .collect()
        })
        .collect()
}

#[derive(Debug)]
struct PositionedNumber {
    i: usize,
    j: usize,
    num: usize,
}

impl PartialEq for PositionedNumber {
    fn eq(&self, other: &PositionedNumber) -> bool {
        self.i == other.i && self.j == other.j && self.num == other.num
    }
}

type Positions = Option<Vec<PositionedNumber>>;

trait Line {
    fn next_i(&self, p: usize) -> usize;
    fn next_j(&self, p: usize) -> usize;
    fn len(&self) -> usize;
    fn positions(&self, matrix: &[&[usize]]) -> Positions {
        let mut pos: Vec<PositionedNumber> = Vec::with_capacity(self.len());

        for p in 0..self.len() {
            let i = self.next_i(p);
            let j = self.next_j(p);
            let row = matrix.get(i)?;
            let num = *row.get(j)?;
            pos.push(PositionedNumber { i, j, num });
        }

        Some(pos)
    }
}

struct DiagLine {
    i: usize,
    j: usize,
    length: usize,
}

impl Line for DiagLine {
    fn next_i(&self, p: usize) -> usize {
        self.i + p
    }
    fn next_j(&self, p: usize) -> usize {
        self.j + p
    }
    fn len(&self) -> usize {
        self.length
    }
}

struct HorizLine {
    i: usize,
    j: usize,
    length: usize,
}

impl Line for HorizLine {
    fn next_i(&self, _p: usize) -> usize {
        self.i
    }
    fn next_j(&self, p: usize) -> usize {
        self.j + p
    }
    fn len(&self) -> usize {
        self.length
    }
}

struct VertLine {
    i: usize,
    j: usize,
    length: usize,
}

impl Line for VertLine {
    fn next_i(&self, p: usize) -> usize {
        self.i + p
    }
    fn next_j(&self, _p: usize) -> usize {
        self.j
    }
    fn len(&self) -> usize {
        self.length
    }
}

fn find_product(table: &str, length: usize) -> usize {
    let matrix = table_to_matrix(table);
    let matrix_slices: Vec<&[usize]> = matrix.iter().map(|row| row.as_slice()).collect();
    let mut highest = 0;
    let mut positions: Vec<PositionedNumber> = Vec::new();

    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let lines: Vec<Box<Line>> = vec![
                Box::new(HorizLine { i, j, length }),
                Box::new(VertLine { i, j, length }),
                Box::new(DiagLine { i, j, length }),
            ];

            for line in lines {
                match line.positions(matrix_slices.as_slice()) {
                    Some(cur_positions) => {
                        let product: usize = cur_positions.iter().map(|x| x.num).product();
                        if product > highest {
                            highest = product;
                            positions = cur_positions;
                        }
                    }
                    _ => (),
                }
            }

        }
    }

    println!("{:?}", positions);

    highest
}

fn main() {
    println!("{}", find_product(SAMPLE, 4));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIXT_DIAG: &str = r#"
03 01 01
01 03 01
01 01 03
    "#;

    const FIXT_VERT: &str = r#"
01 04 01
01 04 01
01 05 01
    "#;

    const FIXT_HORIZ: &str = r#"
02 01 01 01
01 02 01 01
02 02 03 04
01 01 01 01
    "#;

    #[test]
    fn finds_product_diag_3() {
        assert_eq!(find_product(FIXT_DIAG, 3), 27);
    }

    #[test]
    fn finds_product_vert_2() {
        assert_eq!(find_product(FIXT_VERT, 2), 20);
    }

    #[test]
    fn finds_product_horiz_4() {
        assert_eq!(find_product(FIXT_HORIZ, 4), 48);
    }

    #[test]
    fn converts_table_to_matrix() {
        assert_eq!(
            table_to_matrix(FIXT_DIAG),
            vec![vec![3, 1, 1], vec![1, 3, 1], vec![1, 1, 3]]
        );
    }

    #[test]
    fn gets_positions_from_horiz_line() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![
            PositionedNumber { i: 1, j: 1, num: 5 },
            PositionedNumber { i: 1, j: 2, num: 6 },
        ];
        let line = HorizLine {
            i: 1,
            j: 1,
            length: 2,
        };

        let slices: Vec<&[usize]> = matrix.iter().map(|row| row.as_slice()).collect();
        assert_eq!(line.positions(slices.as_slice()).unwrap(), expected);
    }

    #[test]
    fn gets_positions_from_vert_line() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![
            PositionedNumber { i: 1, j: 1, num: 5 },
            PositionedNumber { i: 2, j: 1, num: 8 },
        ];
        let line = VertLine {
            i: 1,
            j: 1,
            length: 2,
        };

        let slices: Vec<&[usize]> = matrix.iter().map(|row| row.as_slice()).collect();
        assert_eq!(line.positions(slices.as_slice()).unwrap(), expected);
    }

    #[test]
    fn gets_positions_from_diag_line() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![
            PositionedNumber { i: 1, j: 1, num: 5 },
            PositionedNumber { i: 2, j: 2, num: 9 },
        ];
        let line = DiagLine {
            i: 1,
            j: 1,
            length: 2,
        };

        let slices: Vec<&[usize]> = matrix.iter().map(|row| row.as_slice()).collect();
        assert_eq!(line.positions(slices.as_slice()).unwrap(), expected);
    }
}
