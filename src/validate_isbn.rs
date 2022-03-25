use std::str::FromStr;

struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
enum InvalidIsbn {
    TooLong,
    TooShort,
    FailedChecksum,
    InvalidCharacter(usize, char),
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NOTE: This does not validate the positions of each character
        // Additional checks based on the position should be present

        // First twelve digits are the actual numbers
        // Last digit is the check digit
        let mut digits = Vec::with_capacity(13);
        for (i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => return Err(InvalidIsbn::InvalidCharacter(i, c)),
            }
        }

        if digits.len() > 13 {
            return Err(InvalidIsbn::TooLong);
        }

        if digits.len() < 13 {
            return Err(InvalidIsbn::TooShort);
        }

        let check_digit = digits.last().unwrap();
        if *check_digit != calculate_check_digit(&digits[..12]) {
            return Err(InvalidIsbn::FailedChecksum);
        }

        Ok(Self {
            digits,
            raw: s.to_string(),
        })
    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    // Multiply each digit by their pre-assigned weights, then sum the result
    // Weights alternate between 1 and 3
    // Afterwards, reduce to a single digit by doing 10 - (sum % 10)
    // - So if remainder is 0, use 0 for check digit

    // NOTE: Another way of implementing this is to create another array containing the pre-defined weights
    // Then zip these together and multiply
    // This makes the rest of the function weight-agnostic

    debug_assert!(digits.len() == 12);

    let sum: u32 = digits
        .iter()
        .enumerate()
        .map(|(i, &digit)| if i % 2 == 0 { digit } else { digit * 3 })
        .map(|subtotal| subtotal as u32)
        .sum();

    let remainder = (sum % 10) as u8;
    match remainder {
        0 => 0,
        rem => 10 - rem,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_correctly_calculate_check_digits() {
        let cases = [
            ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
            ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
        ];

        for (case, check) in cases.iter() {
            let actual = calculate_check_digit(case);
            println!("{:?} -> {}?  {}", &case, check, actual);
            assert_eq!(calculate_check_digit(case), *check)
        }
    }

    #[test]
    fn rust_in_action() {
        let _: Isbn = "978-3-16-148410-0".parse().unwrap();
    }
}
