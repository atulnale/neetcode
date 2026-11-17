impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                if board[i][j] == '.' {
                    continue;
                }
                if set.contains(&board[i][j]) {
                    return false;
                }
                set.insert(board[i][j]);
            }
        }

        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                if board[j][i] == '.' {
                    continue;
                }
                if set.contains(&board[j][i]) {
                    return false;
                }
                set.insert(board[j][i]);
            }
        }
        return Solution::is_valid(&board, 0, 0)
            && Solution::is_valid(&board, 0, 3)
            && Solution::is_valid(&board, 0, 6)
            && Solution::is_valid(&board, 3, 0)
            && Solution::is_valid(&board, 3, 3)
            && Solution::is_valid(&board, 3, 6)
            && Solution::is_valid(&board, 6, 0)
            && Solution::is_valid(&board, 6, 3)
            && Solution::is_valid(&board, 6, 6);
    }

    fn is_valid(board: &Vec<Vec<char>>, starti: usize, startj: usize) -> bool {
        let mut set = HashSet::new();
        for i in starti..starti + 3 {
            for j in startj..startj + 3 {
                if board[i][j] == '.' {
                    continue;
                }
                if set.contains(&board[i][j]) {
                    return false;
                }
                set.insert(&board[i][j]);
            }
        }
        true
    }
}