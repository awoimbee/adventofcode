use heapless::Vec as StackVec;

const INPUT: &str = include_str!("../input/day04.txt");
const MAX_DRAWED: usize = 100;
const MAX_BOARDS: usize = 100;
/// Needs for loops in const functions
/// ```rust
/// let mut rows_columns = [[0; 5]; 10];
/// for i in 0..5 {
///    for j in 0..5 {
///        // Rows
///        rows_columns[i][j] = i * 5 + j;
///        // Columns
///        rows_columns[5 + i][j] = i + j * 5;
///    }
/// }
/// ```
const ROW_COLUMNS: [[usize; 5]; 10] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24],
];

type Int = i32;
// type Board = [Int; 5 * 5];

#[derive(Debug, Clone, Copy)]
struct Board {
    board: [Int; 5 * 5],
    min_rounds: usize,
}

impl Board {
    fn default() -> Self {
        Self {
            board: [0; 5 * 5],
            min_rounds: 5,
        }
    }

    fn set_nb(&mut self, drawed: Int) {
        for nb in self.board.iter_mut() {
            if *nb == drawed {
                *nb = -1;
                return;
            }
        }
    }

    fn has_won(&mut self) -> bool {
        if self.min_rounds > 0 {
            self.min_rounds -= 1;
            return false;
        }
        for candidate in ROW_COLUMNS.iter() {
            let count_bad_nb = candidate
                .iter()
                .filter(|&&nb_idx| self.board[nb_idx] != -1)
                .count();
            if count_bad_nb == 0 {
                return true;
            } else if self.min_rounds == 0 || count_bad_nb < self.min_rounds {
                self.min_rounds = count_bad_nb;
            }
        }
        false
    }
}

struct Parsed {
    drawn: StackVec<Int, MAX_DRAWED>,
    boards: StackVec<Board, MAX_BOARDS>,
}

impl Parsed {
    fn new(input: &str) -> Self {
        let mut in_it = input.trim_end().split("\n\n");
        let drawn = in_it
            .next()
            .unwrap()
            .split(',')
            .map(|nb| nb.parse().unwrap())
            .collect();
        let mut boards = StackVec::<Board, MAX_BOARDS>::new();
        for board_str in in_it {
            boards.push(Board::default()).unwrap();
            let b = boards.last_mut().unwrap();
            let mut i = 0;
            for l in board_str.split('\n') {
                for nb in l
                    .split(' ')
                    .filter(|&s| !s.is_empty())
                    .map(|s| s.parse().unwrap())
                {
                    b.board[i] = nb;
                    i += 1;
                }
            }
        }
        Self { drawn, boards }
    }
}

fn board_score(board: &Board, draw: Int) -> Int {
    board.board.iter().filter(|nb| **nb != -1).sum::<Int>() * draw
}

fn solve(input: &mut Parsed) -> (String, String) {
    let mut first_board = None;
    let mut last_board = None;
    let mut boards_playing = (0..input.boards.len()).collect::<StackVec<_, MAX_BOARDS>>();

    // assuming all boards won == all numbers drawn
    for &d in input.drawn.iter() {
        let mut boards_won = Vec::with_capacity(boards_playing.len());
        for &board_id in boards_playing.iter() {
            let b = input.boards.get_mut(board_id).unwrap();
            b.set_nb(d);
            if b.has_won() {
                boards_won.push((board_id, d));
            }
        }
        for (board_id, draw) in boards_won {
            match first_board {
                None => first_board = Some((board_id, draw)),
                _ => last_board = Some((board_id, draw)),
            }
            boards_playing.swap_remove(boards_playing.iter().position(|x| *x == board_id).unwrap());
        }
    }

    let first = first_board.unwrap();
    let last = last_board.unwrap();

    let part1 = (board_score(&input.boards[first.0], first.1)).to_string();
    let part2 = (board_score(&input.boards[last.0], last.1)).to_string();
    (part1, part2)
}

pub fn day04() -> (String, String) {
    let mut parsed = Parsed::new(INPUT);

    solve(&mut parsed)
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#;

    #[test]
    fn test_part1_test_input() {
        let mut parsed = Parsed::new(TEST_INPUT);
        assert_eq!(solve(&mut parsed).0, "4512".to_string());
    }

    #[test]
    fn test_part2_test_input() {
        let mut parsed = Parsed::new(TEST_INPUT);
        assert_eq!(solve(&mut parsed).1, "1924".to_string());
    }
}
