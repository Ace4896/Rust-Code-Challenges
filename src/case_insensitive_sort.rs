fn sort_usernames<T: AsRef<str>>(users: &mut Vec<T>) {
    users.sort_by_cached_key(|name| name.as_ref().to_lowercase());
}

#[cfg(test)]
mod tests {
    use super::sort_usernames;

    #[test]
    fn five_users() {
        let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
        let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
        sort_usernames(&mut users);

        assert_eq!(users, sorted);
    }
}
