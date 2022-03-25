use std::str::FromStr;

use chrono::NaiveDate;

fn weeks_between(a: &str, b: &str) -> i32 {
    let a = NaiveDate::from_str(a).unwrap();
    let b = NaiveDate::from_str(b).unwrap();
    let diff = b - a;

    diff.num_weeks() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_day() {
        let n_weeks = weeks_between("1010-10-10", "1010-10-10");
        assert_eq!(n_weeks, 0);
    }

    #[test]
    fn one_week() {
        let n_weeks = weeks_between("1010-10-10", "1010-10-18");
        assert_eq!(n_weeks, 1);
    }

    #[test]
    fn past() {
        let n_weeks = weeks_between("1010-10-18", "1010-10-10");
        assert_eq!(n_weeks, -1);
    }
}
