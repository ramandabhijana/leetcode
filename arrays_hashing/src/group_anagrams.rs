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
        todo!()
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
    fn test_case_2() {
        let strs: Vec<String> = ["a".to_owned()].to_vec();
        assert_eq!(Solution::group_anagrams(strs), vec![vec!["a".to_owned()]]);
    }

    // #[test]
    // fn test_case_2() {
    //     let nums: Vec<i32> = [1, 2, 3, 4].into();
    //     assert!(!Solution::group_anagrams(nums));
    // }

    // #[test]
    // fn test_case_3() {
    //     let nums: Vec<i32> = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2].into();
    //     assert!(Solution::group_anagrams(nums));
    // }
}
