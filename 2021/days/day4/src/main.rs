type AnyError = Box<dyn std::error::Error>;

const INPUT: &str = include_str!("../../../inputs/day4");

fn main() -> Result<(), AnyError> {
    let (nums, boards) = INPUT.split_once("\n").expect("missing delimiter");

    let nums = nums
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    let inputs = boards
        .split_ascii_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<i32>, _>>()?;

    let result = {
        let mut inputs = inputs.clone();

        nums.iter().find_map(|num| {
            for board in inputs.chunks_exact_mut(25) {
                mark(board, *num);

                if is_completed(board) {
                    return Some(board.iter().filter(|&&value| value > 0).sum::<i32>() * num);
                }
            }
            None
        })
    };
    dbg!(result);

    let result = {
        let mut inputs = inputs;
        let mut complected_board = 0;
        let nb_boards = inputs.chunks_exact_mut(25).count();

        nums.iter().find_map(|num| {
            for board in inputs
                .chunks_exact_mut(25)
                .filter(|board| !is_completed(board))
            {
                mark(board, *num);
                if is_completed(board) {
                    complected_board += 1;
                    if complected_board == nb_boards {
                        return Some(board.iter().filter(|&&value| value > 0).sum::<i32>() * num);
                    }
                }
            }
            None
        })
    };
    dbg!(result);

    Ok(())
}

fn is_completed(board: &[i32]) -> bool {
    board
        .chunks_exact(5)
        .any(|row| row.iter().all(|&value| value < 0))
        || (0..5).any(|n| board.iter().skip(n).step_by(5).all(|&value| value < 0))
}

fn mark(board: &mut [i32], num: i32) {
    for value in board.iter_mut() {
        if *value == num {
            *value *= -1;
            *value -= 1;
        }
    }
}
