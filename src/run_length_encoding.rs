pub fn encode(text: &str) -> String {
    let mut encoded = String::new();
    let mut chars = text.chars();
    let mut count = 0;
    let mut prev: Option<char> = None;

    while let Some(c) = chars.next() {
        if prev.is_none() {
            prev = Some(c);
        }

        if prev.unwrap() == c && count < 9 {
            count += 1;
        } else {
            encoded.push_str(&format!("{}{}", count, prev.unwrap()));
            prev = Some(c);
            count = 1;
        }
    }

    if prev.is_some() {
        encoded.push_str(&format!("{}{}", count, prev.unwrap()));
    }

    encoded
}

pub fn decode(text: &str) -> String {
    let mut decoded = String::new();
    let mut chars = text.chars();

    while let (Some(n), Some(c)) = (chars.next().map(|c| c.to_digit(10)).flatten(), chars.next()) {
        for _ in 0..n {
            decoded.push(c);
        }
    }

    decoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abc() {
        assert_eq!(encode("abc"), "1a1b1c");
    }
    
    #[test]
    fn round_trip() {
        let input = "LinkedIn";
        println!("{}", encode(input));
        assert_eq!(decode(&encode(input)), input);
    }
    
    #[test]
    fn long_run() {
        let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
        assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
    }
}