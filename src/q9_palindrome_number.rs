pub fn is_palindrome(x: i32) -> bool {
    let s1 = x.to_string();
    let s: Vec<_> = s1.chars().collect();
    let n = s1.len();
    for i in 0..n {
        if s[i] != s[n - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_121() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn example_neg_121() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn example_10() {
        assert!(!is_palindrome(10));
    }
}