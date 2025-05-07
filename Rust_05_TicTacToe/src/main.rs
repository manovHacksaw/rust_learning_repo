use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';

const BOARD_SIZE: usize = 3;
type BOARD =[[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> BOARD {
   return [[' '; BOARD_SIZE]; BOARD_SIZE]
}

fn print_board(board: &BOARD) {
    for row in board.iter() {
        for cell in row.iter() {
            print!("| {} ", cell);
        }
        println!("|");
    }
}

fn get_player_move(current_player: char, board: &BOARD) -> (usize, usize) {
    loop {
        println!("Player {}: Enter your move (row and column): ", current_player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid input. Please enter row and column separated by space.");
            continue;
        }

        let row: usize = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid row. Please enter a number.");
                continue;
            }
        };

        let col: usize = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid column. Please enter a number.");
                continue;
            }
        };

        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            println!("Invalid move. Row and column must be between 0 and {}.", BOARD_SIZE - 1);
            continue;
        }

        if board[row][col] != ' ' {
            println!("Cell already taken. Try again.");
            continue;
        }

        return (row, col);
    }

}

fn play_game(){
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;

    loop{
        print!("Current board:\n");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        current_player = if current_player==PLAYER_X{
            PLAYER_O
        } else {
            PLAYER_X
        }
    }

    
}

fn main(){
    
}