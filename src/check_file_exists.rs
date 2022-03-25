use std::fs;
use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn exists(&self) -> bool {
        self.exists()
    }

    fn is_readable(&self) -> bool {
        fs::File::open(self).is_ok()
    }

    fn is_writeable(&self) -> bool {
        match self.metadata() {
            Ok(metadata) => !metadata.permissions().readonly(),
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile;

    #[test]
    fn writeable() {
        let f = tempfile::NamedTempFile::new().unwrap();
        assert!(f.path().is_writeable());

        fs::remove_file(f.path()).unwrap();
    }

    #[test]
    fn read_only() {
        let f = tempfile::NamedTempFile::new().unwrap();
        let mut perms = fs::metadata(f.path()).unwrap().permissions();

        // Main test
        perms.set_readonly(true);
        fs::set_permissions(f.path(), perms.clone()).unwrap();
        assert_eq!(f.path().is_writeable(), false);

        perms.set_readonly(false);
        fs::set_permissions(f.path(), perms).unwrap();
        assert_eq!(f.path().is_writeable(), true);

        // Cleanup
        fs::remove_file(f.path()).unwrap();
    }
}
