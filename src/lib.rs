pub mod assets {
    pub mod atlas;
    pub mod gamemap;
}

/// Just a function returning 5
///
/// # Examples
///
/// ```
/// use ferari::get_five;
///
/// assert_eq!(get_five(), 5);
/// ```
pub fn get_five() -> i32 {
    5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_five() {
        assert_eq!(get_five(), 5);
    }
}
