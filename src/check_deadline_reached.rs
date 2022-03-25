use chrono::{Date, Local};

struct ImportantEvent {
    what: String,
    when: Date<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        Local::today() >= self.when
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_past() {
        use chrono::Duration;

        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::today() - Duration::hours(25),
        };

        assert!(event.is_passed())
    }

    #[test]
    fn in_future() {
        use chrono::Duration;

        let event = ImportantEvent {
            what: String::from("friend's birthday"),
            when: Local::today() + Duration::hours(25),
        };

        assert!(!event.is_passed())
    }
}
