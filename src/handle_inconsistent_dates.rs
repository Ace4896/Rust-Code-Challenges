use chrono::NaiveDate;

fn is_year(field: &str) -> bool {
    field.len() == 4 && field.chars().all(|c| c.is_ascii_digit())
}

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    // If there are no digits in the input, we can't parse anything
    if !text.chars().any(|c| c.is_ascii_digit()) {
        return None;
    }
    
    let fields: Vec<_> = text.split(['/', '-', '.', ' ']).collect();
    let mut year = None;
    let mut month = None;
    let mut day = None;

    // Look for months that are written in English text
    for field in &fields {
        if field.len() < 3 {
            continue;
        }
        
        let m = match &field.to_lowercase()[..3] {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "may" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => continue,
        };

        month = Some(m);
        break;
    }

    // Look for day and year, and month if necessary
    for field in &fields {
        if is_year(field) {
            year = field.parse::<i32>().ok();
            continue;
        }

        // NOTE: This assumes that the month goes first, but this isn't always the case
        // Extra heuristics are needed to guess which one is the day and which one is the month
        // - If the field is greater than 12, definitely the day field
        // - ... any others?
        if month.is_some() {
            day = field.parse::<u32>().ok();
        } else {
            month = field.parse::<u32>().ok();
        }
    }

    match (year, month, day) {
        (Some(y), Some(m), Some(d)) => NaiveDate::from_ymd_opt(y, m, d),
        (Some(y), Some(m), None) => NaiveDate::from_ymd_opt(y, m, 1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ymd_hyphen() {
        assert_eq!(
            flexible_date_parse("2010-12-11"),
            Some(NaiveDate::from_ymd(2010, 12, 11))
        )
    }

    #[test]
    fn ymd_slash() {
        assert_eq!(
            flexible_date_parse("1999/Mar/02"),
            Some(NaiveDate::from_ymd(1999, 3, 2))
        )
    }

    #[test]
    fn dmy_dot() {
        assert_eq!(
            flexible_date_parse("01.Mar.2021"),
            Some(NaiveDate::from_ymd(2021, 3, 1))
        )
    }

    #[test]
    fn mdy_dot() {
        assert_eq!(
            flexible_date_parse("Apr.05.2021"),
            Some(NaiveDate::from_ymd(2021, 4, 5))
        )
    }

    #[test]
    fn invalid() {
        assert_eq!(flexible_date_parse("not a date"), None)
    }
}
