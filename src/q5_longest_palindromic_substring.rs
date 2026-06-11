pub fn longest_palindrome(s: String) -> String {
    let mut max_len = 1;
    let mut start = 0;
    let n = s.len();
    for i in 0..n {
        for j in i..n {
            if (j - i + 1) > max_len && Self::is_palindrome(&s, i, j) {
                start = i;
                max_len = j - i + 1;
            }
        }
    }
    s[start..start + max_len].to_string()
}

fn is_palindrome(s: &str, left: usize, right: usize) -> bool {
    let s = s.as_bytes();
    let mut l = left;
    let mut r = right;

    while l < r {
        if s[l] != s[r] {
            return false;
        }
        l += 1;
        r -= 1;
    }
    true
}