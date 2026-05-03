/// Brand voice string templates. Use with format!().
/// Example: voice::COPIED.replace("{account}", "github")
pub const ADDED_OK:  &str = "Sir Wink's got it. 😉";
pub const COPIED:    &str = "Copied · {account}";
pub const NOT_FOUND: &str = "no account named \"{name}\" — did you mean {suggestion}?";
pub const NO_MATCH:  &str = "no accounts match \"{query}\"";
pub const REMOVED:   &str = "Removed {account}.";
pub const RENAMED:   &str = "Renamed {old} → {new}.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copied_formats_correctly() {
        let msg = COPIED.replace("{account}", "github");
        assert_eq!(msg, "Copied · github");
    }

    #[test]
    fn not_found_formats_correctly() {
        let msg = NOT_FOUND
            .replace("{name}", "notion")
            .replace("{suggestion}", "notion-work");
        assert_eq!(msg, "no account named \"notion\" — did you mean notion-work?");
    }
}
