#[derive(Debug)]
struct Date {
    year: u32,
    month: u8,
    day: u8,
    week_day: u8,
}

impl Date {
    fn new() -> Date {
        Date {
            year: 1900,
            month: 1,
            day: 1,
            week_day: 2,
        }
    }

    fn add(&mut self) {
        self.day += 1;
        self.week_day += 1;

        if self.week_day > 7 {
            self.week_day = 1;
        }

        if self.day > end_of_month(self.year, self.month) {
            self.month += 1;
            self.day = 1;
        }

        if self.month > 12 {
            self.year += 1;
            self.month = 1;
        }
    }
}

fn end_of_month(year: u32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) {
                29
            } else {
                28
            }
        }
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Date) -> bool {
        if self.year != other.year || self.month != other.month || self.day != other.day ||
            self.week_day != other.week_day
        {
            return false;
        }
        true
    }
}

fn main() {
    let mut count = 0;
    let mut date = Date {
        year: 1901,
        month: 1,
        day: 1,
        week_day: 3,
    };
    let end_date = Date {
        year: 2000,
        month: 12,
        day: 31,
        week_day: 1,
    };

    while date != end_date {
        if date.day == 1 && date.week_day == 1 {
            count += 1;
        }
        date.add();
    }

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_new_date() {
        let date = Date::new();

        assert_eq!(date.year, 1900);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 1);
        assert_eq!(date.week_day, 2);
    }

    #[test]
    fn adds_one_day() {
        let mut date = Date::new();

        date.add();

        assert_eq!(date.year, 1900);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 2);
        assert_eq!(date.week_day, 3);
    }

    #[test]
    fn equals_to_date() {
        let mut date1 = Date::new();
        let mut date2 = Date::new();

        date1.add();
        date2.add();

        assert_eq!(date1, date2);
    }

    #[test]
    fn unequals_to_date() {
        let mut date1 = Date::new();
        let date2 = Date::new();

        date1.add();

        assert_ne!(date1, date2);
    }

    #[test]
    fn adds_7_days() {
        let mut date = Date::new();

        for _ in 1..8 {
            date.add();
        }

        assert_eq!(
            date,
            Date {
                year: 1900,
                month: 1,
                day: 8,
                week_day: 2,
            }
        );
    }

    #[test]
    fn adds_31_days() {
        let mut date = Date::new();

        for _ in 1..32 {
            date.add();
        }

        assert_eq!(
            date,
            Date {
                year: 1900,
                month: 2,
                day: 1,
                week_day: 5,
            }
        );
    }

    #[test]
    fn gets_end_of_months() {
        assert_eq!(end_of_month(1900, 1), 31);
        assert_eq!(end_of_month(1900, 2), 28);
        assert_eq!(end_of_month(1900, 3), 31);
        assert_eq!(end_of_month(1900, 4), 30);
        assert_eq!(end_of_month(1900, 5), 31);
        assert_eq!(end_of_month(1900, 6), 30);
        assert_eq!(end_of_month(1900, 7), 31);
        assert_eq!(end_of_month(1900, 8), 31);
        assert_eq!(end_of_month(1900, 9), 30);
        assert_eq!(end_of_month(1900, 10), 31);
        assert_eq!(end_of_month(1900, 11), 30);
        assert_eq!(end_of_month(1900, 12), 31);
    }

    #[test]
    fn gets_leap_years() {
        assert_eq!(end_of_month(1904, 2), 29);
        assert_eq!(end_of_month(1908, 2), 29);
        assert_eq!(end_of_month(2000, 2), 29);
        assert_eq!(end_of_month(1800, 2), 28);
    }

    #[test]
    fn adds_365_days() {
        let mut date = Date::new();

        for _ in 1..366 {
            date.add();
        }

        assert_eq!(
            date,
            Date {
                year: 1901,
                month: 1,
                day: 1,
                week_day: 3,
            }
        );
    }
}
