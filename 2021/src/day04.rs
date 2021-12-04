use heapless::Vec as FastVec;

const INPUT: &str = include_str!("../input/day04.txt");
const MAX_DRAWED: usize = 100;
const MAX_BOARDS: usize = 100;

struct Parsed {
    drawn: FastVec<i32, MAX_DRAWED>,
    boards: FastVec<ndarray::Array2<i32>, MAX_BOARDS>,
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
        let boards = in_it
            .map(|board_str| {
                board_str
                    .split('\n')
                    .flat_map(|l| {
                        l.split(' ')
                            .filter(|&x| !x.is_empty())
                            .map(|s| s.parse::<i32>().unwrap())
                    })
                    .collect::<ndarray::Array1<_>>()
                    .into_shape((5, 5))
                    .unwrap()
            })
            .collect();
        Self { drawn, boards }
    }
}

fn board_score(board: &ndarray::Array2<i32>, draw: i32) -> i32 {
    board.iter().filter(|nb| **nb != -1).sum::<i32>() * draw
}

fn solve(input: &mut Parsed) -> (String, String) {
    let mut boards_won = Vec::<(usize, i32)>::with_capacity(MAX_BOARDS);
    let mut boards_playing = (0..input.boards.len()).collect::<Vec<usize>>();

    // assuming all boards won == all numbers drawn
    for d in input.drawn.iter() {
        // println!("Drawn");
        for board_id in boards_playing.clone().iter() {
            // println!("  Board");
            let b = input.boards.get_mut(*board_id).unwrap();
            for r in b.rows_mut().into_iter() {
                let mut fail = false;
                for a in r {
                    match a {
                        -1 => continue,
                        _ if a == d => *a = -1,
                        _ => {
                            fail = true;
                            continue;
                        }
                    }
                }
                if !fail {
                    boards_won.push((*board_id, *d));
                    boards_playing.retain(|&x| x != *board_id);
                }
            }
            // At this point we already iterated on every number
            'column: for c in b.columns_mut().into_iter() {
                for a in c {
                    match a {
                        -1 => continue,
                        _ => continue 'column,
                    }
                }
                boards_won.push((*board_id, *d));
                boards_playing.retain(|&x| x != *board_id);
            }
        }
    }

    let first = boards_won.first().unwrap();
    let last = boards_won.last().unwrap();

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
