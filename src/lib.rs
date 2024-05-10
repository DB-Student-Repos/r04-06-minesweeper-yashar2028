pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    if rows == 0 {
        return vec![];
    }
    let columns = minefield[0].len();
    let directions = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
    let mut output_new_board = vec!["".to_string(); rows];
   
    for row in 0..rows {
        for column in 0..columns {
            let cell = minefield[row].as_bytes()[column];
            if cell == b'*' {
                output_new_board[row].push('*');
                continue;
            }
            let mut mine_count = 0;
            for &(dx, dy) in &directions {
                let new_row = (row as isize) + dx;
                let new_col = (column as isize) + dy;
                if new_row >= 0 && new_row < rows as isize &&
                   new_col >= 0 && new_col < columns as isize &&
                   minefield[new_row as usize].as_bytes()[new_col as usize] == b'*' {
                    mine_count += 1;
                }
            }
            if mine_count > 0 {
                output_new_board[row].push_str(&mine_count.to_string());
            } else {
                output_new_board[row].push_str(" ");     
            }
        }
    }
    output_new_board
}


