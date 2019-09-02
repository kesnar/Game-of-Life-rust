use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let width:usize = args[1].parse::<usize>().unwrap();
    let height:usize = args[2].parse::<usize>().unwrap();

    /*println!("width = {}", width);
    println!("height = {}", height);*/

    let board = random_board(width, height);
    render(&board, width, height)
}

/// Creates a board with every cell dead
fn dead_board(width: usize, height:usize) -> Vec<Vec<bool>> {
    let board: Vec<Vec<bool>> = vec![vec![false; width];height];

    return board
}

///Creates a board with randomized cell states
fn random_board(width: usize, height:usize) -> Vec<Vec<bool>> {
    let mut board = dead_board(width, height);

    for i in 0..width {
        for j in 0..height {
            board[i][j] = match rand::thread_rng().gen_range(0,2) {
                0 => false,
                1 => true,
                _ => panic!("I don't know when this happens! :("),
            }
        }
    };

    return board
}

///Creates a string that is the representation of the board and prints it
fn render(board: &[Vec<bool>], width: usize, height:usize) {

    let mut rend = String::new();
    rend.push_str("+-");
    for _i in 0..width {
        rend.push_str("--");
    }
    rend.push_str("+\n");

    for i in 0..width {
        rend.push_str("| ");
        for j in 0..height {
            if board[i][j] {
                rend.push_str("# ");
            }
            else {
                rend.push_str(". ");
            }
        }
        rend.push_str("|\n");
    }

    rend.push_str("+-");
    for _j in 0..height {
        rend.push_str("--");
    }
    rend.push_str("+\n");
    print!("{}", rend);

}

fn next_state(current: bool, neighborhood: i32) -> bool {
    if current {
        match neighborhood{
            2 => true,
            3 => true,
            _ => false
        }
    }
    else {
        match neighborhood{
            3 => true,
            _ => false
        }
    }
}

fn next_board(board: &[Vec<bool>], width: usize, height:usize) -> Vec<Vec<bool>>{
    let mut next_board_state: Vec<Vec<bool>> = vec![vec![false; width];height];

    for i in 0..width {
        for j in 0..height {
            let neighborhood: i32 = 0;

            next_board_state[i][j] = next_state(board[i][j], neighborhood);
        }
    }

    next_board_state
}