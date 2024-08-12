/// Group Anagrams
/// Given an array of strings strs, group the anagrams together. You can return the answer in any order.
///
/// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
///
/// Example 1:
///     Input: strs = ["eat","tea","tan","ate","nat","bat"]
///     Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
///
/// Example 2:
///     Input: strs = [""]
///     Output: [[""]]
///
/// Example 3:
///     Input: strs = ["a"]
///     Output: [["a"]]
///
/// Constraints:
///     1 <= strs.length <= 104
///     0 <= strs[i].length <= 100
///     strs[i] consists of lowercase English letters.
///

struct Solution;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 1 {
            return vec![strs];
        }

        use std::collections::HashMap;

        let mut grouped_word: HashMap<String, Vec<String>> = HashMap::new();

        for word in strs {
            let mut sorted_word_chars: Vec<char> = word.chars().collect();
            sorted_word_chars.sort_by(|lhs, rhs| lhs.cmp(rhs));

            let key: String = sorted_word_chars.into_iter().collect();
            grouped_word
                .entry(key)
                .and_modify(|words| words.push(word.clone()))
                .or_insert(vec![word]);
        }

        let grouped_word: Vec<Vec<String>> = grouped_word.into_iter().map(|(_, v)| v).collect();
        grouped_word
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let strs: Vec<String> = ["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        // Since the order doesnt matter, the assertion may fail
        assert_eq!(
            Solution::group_anagrams(strs),
            vec![
                vec!["bat".to_owned()],
                vec!["nat".to_owned(), "tan".to_owned()],
                vec!["ate".to_owned(), "eat".to_owned(), "tea".to_owned()]
            ]
        );
    }

    #[test]
    fn test_case_2() {
        let strs: Vec<String> = [String::new()].to_vec();
        assert_eq!(Solution::group_anagrams(strs), vec![vec![String::new()]]);
    }

    #[test]
    fn test_case_3() {
        let strs: Vec<String> = ["a".to_owned()].to_vec();
        assert_eq!(Solution::group_anagrams(strs), vec![vec!["a".to_owned()]]);
    }
}
