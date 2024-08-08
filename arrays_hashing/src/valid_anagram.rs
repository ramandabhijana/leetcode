/// Valid Anagram
/// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
///
/// Example 1:
///     Input: s = "anagram", t = "nagaram"
///     Output: true
/// Example 2:
///     Input: s = "rat", t = "car"
///     Output: false
///
/// Constraints:
///     1 <= s.length, t.length <= 5 * 104
///     s and t consist of lowercase English letters.

struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
    }

    #[test]
    fn test_case_2() {
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}
