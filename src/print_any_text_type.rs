fn info<T: std::fmt::Display>(text: &T) {
    println!("{}", text);
}

#[cfg(test)]
mod tests {
    use super::info;

    #[test]
    fn str() {
        let input = "Rust";
        info(&input);
    }

    #[test]
    fn string() {
        let input = String::from("Rust");
        info(&input);
    }

    #[test]
    fn chars() {
        let input = 'r';
        info(&input);
    }

    #[test]
    fn cstring() {
        use std::ffi::CString;

        let input = CString::new("Rust").unwrap();
        info(&input.to_str().unwrap());
    }

    #[test]
    fn path() {
        use std::path::Path;
        let input = Path::new("/tmp/rust");
        info(&input.display());
    }
}
