#[cfg(test)]
mod tests {
    use crate::scale::ts_status;

    use super::*;

    #[test]
    fn command_present() {
        assert!(!ts_status().is_empty());
    }
}
