use std::ops::Rem;

const MINE: u8 = "*".as_bytes()[0];
const SPACE: u8 = " ".as_bytes()[0];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    println!("input:\n{:?}", minefield);

    let rows = minefield.len();
    if rows == 0 {
        return Vec::new();
    }

    let columns = minefield[0].len();
    if columns == 0 {
        return vec![String::from("")];
    }

    let board = create_game_board(minefield, rows, columns);
    println!("game board:\n{:?}", board);

    let mut result: Vec<String> = Vec::with_capacity(rows);
    let mut row = String::from("");
    for i in 0..board.len() {
        let mine_count = count_mines_around_position(&board, i, columns);
        row.push_str(&mine_count);

        // reached end of row
        if i.rem(columns) == columns - 1 {
            result.push(row);
            row = String::from("");
            println!("result so far:\n{:?}", result);
        }
    }

    result
}

fn create_game_board(minefield: &[&str], rows: usize, columns: usize) -> Vec<u8> {
    let mut board: Vec<u8> = Vec::with_capacity(rows * columns);
    for row in minefield.iter() {
        for &ch in row.as_bytes().iter() {
            board.push(ch);
        }
    }

    board
}

fn count_mines_around_position(board: &[u8], i: usize, columns: usize) -> String {
    let mut sum = 0;

    if let Some(&ch) = board.get(i) {
        if ch == SPACE {
            let is_start_of_row = i.rem(columns) == 0;
            let is_end_of_row = (i + 1).rem(columns) == 0;
            let is_first_row = i < columns;
            let is_last_row = i > board.len() - columns;

            if !is_start_of_row {
                sum += count_mine_at_index(board, i - 1);

                if !is_first_row {
                    sum += count_mine_at_index(board, i - columns - 1);
                }
                if !is_last_row {
                    sum += count_mine_at_index(board, i + columns - 1);
                }
            }
            if !is_end_of_row {
                sum += count_mine_at_index(board, i + 1);

                if !is_first_row {
                    sum += count_mine_at_index(board, i - columns + 1);
                }
                if !is_last_row {
                    sum += count_mine_at_index(board, i + columns + 1);
                }
            }

            if !is_first_row {
                sum += count_mine_at_index(board, i - columns);
            }

            if !is_last_row {
                sum += count_mine_at_index(board, i + columns);
            }
        } else {
            return char::from_u32(ch.into()).unwrap().to_string();
        }
    }

    if sum == 0 {
        String::from(" ")
    } else {
        sum.to_string()
    }
}

fn count_mine_at_index(board: &[u8], index: usize) -> u8 {
    match board.get(index) {
        Some(&ch) => {
            if ch == MINE {
                1
            } else {
                0
            }
        }
        None => 0,
    }
}
