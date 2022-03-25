use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
enum InvalidRgb {
    MissingLeadingHash,
    InvalidLength,
    InvalidHexDigit,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

impl FromStr for Rgb {
    type Err = InvalidRgb;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = match s.trim().strip_prefix('#') {
            Some(stripped) => stripped,
            _ => return Err(InvalidRgb::MissingLeadingHash),
        };

        if s.len() != 6 {
            return Err(InvalidRgb::InvalidLength);
        }

        if !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(InvalidRgb::InvalidHexDigit);
        }

        // NOTE: Unwrap is safe here since we've verified that s is now a string containing 6 hex digits
        Ok(Rgb {
            r: u8::from_str_radix(&s[0..2], 16).unwrap(),
            g: u8::from_str_radix(&s[2..4], 16).unwrap(),
            b: u8::from_str_radix(&s[4..6], 16).unwrap(),
        })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_color() {
        let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);
    
        for ((r, g), b) in colors {
            let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
            let color: Rgb = hex.parse().unwrap();
            assert_eq!(hex, format!("{}", color));
        }
    }
    #[test]
    #[should_panic]
    fn too_short() {
        let _: Rgb = "#1234".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn not_a_hex_code() {
        let _: Rgb = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_literals() {
        let _: Rgb = "?".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn no_leading_hash() {
        let _: Rgb = "aabbcc".parse().unwrap();
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let _: Rgb = "#00gg00".parse().unwrap();
    }
}
