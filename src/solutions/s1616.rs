use crate::util::solution::Solution;

impl Solution {
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        if a.len() <= 1 {
            return true;
        }
        let a_ch = a.as_bytes();
        let b_ch = b.as_bytes();
        return Solution::check_combine(&a_ch, &b_ch) || Solution::check_combine(&b_ch, &a_ch);
    }

    fn check_combine(a: &[u8], b: &[u8]) -> bool {
        let mut i = 0;
        let mut j = b.len() - 1;
        while i < j && a[i] == b[j] {
            i += 1;
            j -= 1;
        }
        if i >= j {
            return true;
        }
        // we can also form a palindrome if another string without largest suffix and its corresponding prefix is also a palindrome
        return Solution::is_palindrome(a, i, j) || Solution::is_palindrome(b, i, j);
    }

    fn is_palindrome(s: &[u8], l: usize, r: usize) -> bool {
        let mut left = l;
        let mut right = r;
        unsafe {
            while left < right && s.get_unchecked(left) == s.get_unchecked(right) {
                left += 1;
                right -= 1;
            }
        }
        return left >= right;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1616() {
        assert_eq!(
            Solution::check_palindrome_formation(String::from(""), String::from("")),
            true
        );
        assert_eq!(
            Solution::check_palindrome_formation(String::from("x"), String::from("y")),
            true
        );
        assert_eq!(
            Solution::check_palindrome_formation(
                String::from("abcghgbba"),
                String::from("ascdfdcba")
            ),
            true
        );
        assert_eq!(
            Solution::check_palindrome_formation(String::from("abc"), String::from("efg")),
            false
        );
    }
}
