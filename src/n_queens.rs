fn n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut board = vec![0; n];
    let mut solutions = Vec::new();
    n_queens_helper(&mut board, 0, &mut solutions);
    solutions
}

fn n_queens_helper(board: &mut Vec<usize>, row: usize, solutions: &mut Vec<Vec<usize>>) {
    if row == board.len() {
        solutions.push(board.clone());
        return;
    }

    for col in 0..board.len() {
        if is_safe(board, row, col) {
            board[row] = col;
            n_queens_helper(board, row + 1, solutions);
        }
    }
}

fn is_safe(board: &Vec<usize>, row: usize, col: usize) -> bool {
    for (r, c) in board.iter().enumerate().take(row) {
        if c == &col || (r as i32 - row as i32).abs() == (*c as i32 - col as i32).abs() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_queens() {
        let n = 4;
        let solutions = n_queens(n);
        assert_eq!(solutions.len(), 2);
        assert_eq!(solutions[0], vec![1, 3, 0, 2]);
        assert_eq!(solutions[1], vec![2, 0, 3, 1]);
    }

    #[test]
    fn test_n_queens_8() {
        let n = 8;
        let solutions = n_queens(n);
        assert_eq!(solutions.len(), 92);
    }
}