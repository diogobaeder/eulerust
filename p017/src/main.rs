const LESS_10: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];
const LESS_20: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const LESS_100: [&str; 8] = [
    "twenty",
    "thirty",
    "fourty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

fn extend(n: usize) -> String {
    if n < 10 {
        return String::from(LESS_10[n - 1]);
    }
    if n < 20 {
        return String::from(LESS_20[n - 10]);
    }
    if n < 100 {
        let mut tens = String::from(LESS_100[(n / 10) - 2]);
        if n % 10 != 0 {
            let unit = extend(n % 10);
            tens.push_str(format!("-{}", unit).as_str());
        }
        return tens;
    }
    if n < 1000 {
        let hundred_unit = extend(n / 100);
        let mut hundreds = format!("{} hundred", hundred_unit);
        if n % 100 != 0 {
            let rest = extend(n % 100);
            hundreds.push_str(format!(" and {}", rest).as_str());
        }
        return hundreds;
    }
    String::from("one thousand")
}

fn clean(s: &str) -> String {
    String::from(s).replace("-", "").replace(" ", "")
}

fn sum(from: usize, to: usize) -> usize {
    (from..to + 1)
        .map(|x| clean(extend(x).as_str()).len())
        .sum()
}

fn main() {
    println!("{}", sum(1, 1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_1() {
        assert_eq!(extend(1), String::from("one"));
    }

    #[test]
    fn extend_2() {
        assert_eq!(extend(2), String::from("two"));
    }

    #[test]
    fn extend_3() {
        assert_eq!(extend(3), String::from("three"));
    }

    #[test]
    fn extend_10() {
        assert_eq!(extend(10), String::from("ten"));
    }

    #[test]
    fn extend_11() {
        assert_eq!(extend(11), String::from("eleven"));
    }

    #[test]
    fn extend_19() {
        assert_eq!(extend(19), String::from("nineteen"));
    }

    #[test]
    fn extend_20() {
        assert_eq!(extend(20), String::from("twenty"));
    }

    #[test]
    fn extend_21() {
        assert_eq!(extend(21), String::from("twenty-one"));
    }

    #[test]
    fn extend_22() {
        assert_eq!(extend(22), String::from("twenty-two"));
    }

    #[test]
    fn extend_32() {
        assert_eq!(extend(32), String::from("thirty-two"));
    }

    #[test]
    fn extend_100() {
        assert_eq!(extend(100), String::from("one hundred"));
    }

    #[test]
    fn extend_195() {
        assert_eq!(extend(195), String::from("one hundred and ninety-five"));
    }

    #[test]
    fn extend_395() {
        assert_eq!(extend(395), String::from("three hundred and ninety-five"));
    }

    #[test]
    fn extend_317() {
        assert_eq!(extend(317), String::from("three hundred and seventeen"));
    }

    #[test]
    fn extend_1000() {
        assert_eq!(extend(1000), String::from("one thousand"));
    }

    #[test]
    fn cleans_1() {
        assert_eq!(clean("one"), String::from("one"));
    }

    #[test]
    fn cleans_25() {
        assert_eq!(clean("twenty-five"), String::from("twentyfive"));
    }

    #[test]
    fn cleans_125() {
        assert_eq!(
            clean("one hundred and twenty-five"),
            String::from("onehundredandtwentyfive")
        );
    }

    #[test]
    fn sums_20_22() {
        assert_eq!(
            sum(20, 22),
            format!("{}{}{}", "twenty", "twentyone", "twentytwo").len()
        );
    }
}
