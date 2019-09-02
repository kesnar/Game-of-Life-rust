use std::{env, thread, time};
use rand::Rng;


fn main() {
    let args: Vec<String> = env::args().collect();
    let width:usize = args[1].parse::<usize>().unwrap();
    let height:usize = args[2].parse::<usize>().unwrap();

    /*println!("width = {}", width);
    println!("height = {}", height);*/

    let mut board = random_board(width, height);
    loop {
        print!("{}[2J", 27 as char);
        render(&board, width, height);
        thread::sleep(time::Duration::from_millis(100));
        board = next_board(&board, width, height);
    }

    
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

///Calculates next state of a cell (current) based on its neighborhood
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

///Calculates the next state of the board
fn next_board(board: &[Vec<bool>], width: usize, height:usize) -> Vec<Vec<bool>>{
    let mut next_board_state: Vec<Vec<bool>> = vec![vec![false; width];height];

    for i in 0..width {
        for j in 0..height {
            let mut neighborhood: i32 = 0;
            
            if (i!=0) && (j!=0) && (i!=(width-1)) && (j!=(height-1)) {
                        neighborhood = board[i-1][j-1] as i32 + board[i-1][j] as i32 + board[i-1][j+1] as i32 + board[i][j-1] as i32 + board[i][j+1] as i32 + board[i+1][j-1] as i32 + board[i+1][j] as i32 + board[i+1][j+1] as i32;
                    }
                    else if i==0 {
                        if j!=0 {
                            if j!=(height-1) {
                                neighborhood = board[i][j-1] as i32 + board[i][j+1] as i32 + board[i+1][j-1] as i32 + board[i+1][j] as i32 + board[i+1][j+1] as i32;
                            }
                            else {
                                neighborhood = board[i][j-1] as i32 + board[i+1][j-1] as i32 + board[i+1][j] as i32;
                            }
                        }
                        else {
                            neighborhood = board[i][j+1] as i32 + board[i+1][j+1] as i32 + board[i+1][j] as i32;
                        }
                    }
                    else if i==width-1 {
                        if j!=0 {
                            if j!=(height-1) {
                                neighborhood = board[i][j-1] as i32 + board[i][j+1] as i32 + board[i-1][j-1] as i32 + board[i-1][j] as i32 + board[i-1][j+1] as i32;
                            }
                            else {
                                neighborhood = board[i][j-1] as i32 + board[i-1][j-1] as i32 + board[i-1][j] as i32;
                            }
                        }
                        else {
                            neighborhood = board[i][j+1] as i32 + board[i-1][j+1] as i32 + board[i-1][j] as i32;
                        }
                    }
                    else if j==0 {
                        //(i!=0) && (i!=width-1)
                        neighborhood = board[i-1][j] as i32 + board[i-1][j+1] as i32 + board[i][j+1] as i32 + board[i+1][j] as i32 + board[i+1][j+1] as i32;
                    }
                    else if j==(height-1) {
                        //(i!=0) && (i!=width-1)
                        neighborhood = board[i-1][j-1] as i32 + board[i-1][j] as i32 + board[i][j-1] as i32 + board[i+1][j-1] as i32 + board[i+1][j] as i32;
                    }

            next_board_state[i][j] = next_state(board[i][j], neighborhood);
        }
    }

    next_board_state
}

#[test]
fn check_next_state() {
    //TEST 1: dead cells with no live neighbors should stay dead.
    let init_state1 = vec![
        vec![false,false,false],
        vec![false,false,false],
        vec![false,false,false]
    ];
    
    let expected_next_state1 = vec![
        vec![false,false,false],
        vec![false,false,false],
        vec![false,false,false]
    ];

    assert_eq!(expected_next_state1, next_board(&init_state1, 3, 3));

    //TEST 2: dead cells with exactly 3 neighbors should come alive.
    let init_state2 = vec![
        vec![false,false,true],
        vec![false,true,true],
        vec![false,false,false]
    ];
    let expected_next_state2 = vec![
        vec![false,true,true],
        vec![false,true,true],
        vec![false,false,false]
    ];
    assert_eq!(expected_next_state2, next_board(&init_state2, 3, 3));
}