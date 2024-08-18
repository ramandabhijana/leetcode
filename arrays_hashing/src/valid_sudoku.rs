/// Valid Sudoku
/// Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
///     1. Each row must contain the digits 1-9 without repetition.
///     2. Each column must contain the digits 1-9 without repetition.
///     3. Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
/// Note:
///     A Sudoku board (partially filled) could be valid but is not necessarily solvable.
///     Only the filled cells need to be validated according to the mentioned rules.
///
/// Example 1:
///     Input: board =
///     [['5','3','.','.','7','.','.','.','.'],
///      ['6','.','.','1','9','5','.','.','.'],
///      ['.','9','8','.','.','.','.','6','.'],
///      ['8','.','.','.','6','.','.','.','3'],
///      ['4','.','.','8','.','3','.','.','1'],
///      ['7','.','.','.','2','.','.','.','6'],
///      ['.','6','.','.','.','.','2','8','.'],
///      ['.','.','.','4','1','9','.','.','5'],
///      ['.','.','.','.','8','.','.','7','9']]
///     Output: true
///
/// Example 2:
///     Input: board =
///     [['8','3','.','.','7','.','.','.','.'],
///      ['6','.','.','1','9','5','.','.','.'],
///      ['.','9','8','.','.','.','.','6','.'],
///      ['8','.','.','.','6','.','.','.','3'],
///      ['4','.','.','8','.','3','.','.','1'],
///      ['7','.','.','.','2','.','.','.','6'],
///      ['.','6','.','.','.','.','2','8','.'],
///      ['.','.','.','4','1','9','.','.','5'],
///      ['.','.','.','.','8','.','.','7','9']]
///     Output: false
///     Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8.
///                  Since there are two 8's in the top left 3x3 sub-box, it is invalid.
///
/// Constraints:
///     board.length == 9
///     board[i].length == 9
///     board[i][j] is a digit 1-9 or '.'.

struct Solution;
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashMap;

        let mut cols: HashMap<i32, Vec<char>> = HashMap::new();
        let mut rows: HashMap<i32, Vec<char>> = HashMap::new();

        let mut squares: HashMap<(i32, i32), Vec<char>> = HashMap::new();

        // 9 by 9 board
        for row in 0..9 {
            for col in 0..9 {
                // if the cell is not filled proceed to next column
                let cell = board[row][col];
                if cell == '.' {
                    continue;
                }

                let col = col as i32;
                let row = row as i32;

                if rows.get(&row).is_some_and(|rows| rows.contains(&cell))
                    || cols.get(&col).is_some_and(|cols| cols.contains(&cell))
                    || squares
                        .get(&(row / 3, col / 3))
                        .is_some_and(|squares| squares.contains(&cell))
                {
                    return false;
                }

                cols.entry(col)
                    .and_modify(|cols| cols.push(cell))
                    .or_insert(vec![cell]);
                rows.entry(row)
                    .and_modify(|rows| rows.push(cell))
                    .or_insert(vec![cell]);
                squares
                    .entry((row / 3, col / 3))
                    .and_modify(|squares| squares.push(cell))
                    .or_insert(vec![cell]);
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test_case_1() {
        assert!(Solution::is_valid_sudoku(
            [
                ['5', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec()
            ]
            .to_vec()
        ));
    }

    #[test]
    fn test_case_2() {
        assert!(!Solution::is_valid_sudoku(
            [
                ['8', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
                ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
                ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
                ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
                ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
                ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
                ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
                ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
                ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec()
            ]
            .to_vec()
        ));
    }
}
