use std::io;
mod logger;
use log::info;
#[allow(unused_mut)]
#[allow(unused_variables)]

trait StrExt {
    fn remove_last(&self) -> &str;
}

impl StrExt for str {
    fn remove_last(&self) -> &str {
        match self.char_indices().next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
}

fn intro() {
    println!("Hello and welcome to this inpleentation of tic tac toe");
    println!();
    print!("Rules: Player 1 and player 2, represented by X and O, take turns ");
    print!("marking the spaces in a 3*3 grid. The player who succeeds in placing ");
    println!("three of their marks in a horizontal, vertical, or diagonal row wins.");
    println!("Press enter to continue.");
    let mut no_data: String= "".into();
    io::stdin()
        .read_line(&mut no_data) 
        .expect("Failed to read line");
    std::env::remove_var(&mut no_data); // wait for the output, then delete it
    println!();
}

fn create_board() -> Vec<Vec<String>> {
    println!("Here is the playboard: ");
    let board: Vec<Vec<String>>=vec![
        vec![" ".to_string(), " ".to_string(), " ".to_string()],
        vec![" ".to_string(), " ".to_string(), " ".to_string()],
        vec![" ".to_string(), " ".to_string(), " ".to_string()]
    ];
    return board;
}

fn sym() -> (String, String) {
    println!("Player 1, do you want to be X or O?");
    let mut symbol_1: String = String::new();
    io::stdin()
        .read_line(&mut symbol_1) 
        .expect("Failed to read line");

    let mut symbol_2: String = String::new();

    if symbol_1.to_ascii_uppercase()=="X" {
        symbol_1 = "X".to_string();
        symbol_2 = "O".to_string();
    } else {
        symbol_1 = "O".to_string();
        symbol_2 = "X".to_string();
    };
    println!("press enter to continue");
    println!();
    let mut no_data: String= "".into();
    io::stdin()
        .read_line(&mut no_data) 
        .expect("Failed to read line");
    std::env::remove_var(&mut no_data); // wait for the output, then delete it
    println!("\n");
    return (symbol_1, symbol_2)
}

fn get_move() -> (i32, i32) {
    
    println!("Pick a column: {}",
    "[left column: enter 0, middle column: enter 1, right column enter 2]: "
);

    let mut column_str: String = String::new();
    io::stdin()
        .read_line(&mut column_str) 
        .expect("Failed to read line");
    println!();

    let column: i32 = column_str.remove_last().parse::<i32>().unwrap();

    println!("Pick a row: {}",
        "[upper row: enter 0, middle row: enter 1, bottom row: enter 2]: "
    );

    let mut row_str = String::new();
    io::stdin()
        .read_line(&mut row_str) 
        .expect("Failed to read line");

    let row: i32 = row_str.remove_last().parse::<i32>().unwrap();

    println!();
    return (row, column)
}

fn start_gamming (
    mut board: Vec<Vec<String>>, 
    symbol_1: String, 
    symbol_2: String, 
    count: i32
) -> Vec<Vec<String>> {
    // this function starts the game

    let mut player: String=String::new();
    // figures who's turn it is
    if count % 2 == 0 {
        player = symbol_2;
    } else {
        player = symbol_1;
    };

    println!("player {player}, it is your turn.");

    let (mut row, mut column) = get_move();

    while 
        (row > 2 || row < 0) || 
        (column > 2 || column < 0) || 
        (board[row as usize][column as usize]!=" ".to_string())
        {
        if board[row as usize][column as usize]!=" ".to_string() {
            println!("The square you picked is already filled. Pick another one.");
        } else {
            println!("Your move is out of a border, pick another one.");
        }

        (row, column) = get_move();
    };

    board[row as usize][column as usize]=player;

    return board;
}

fn print_pretty(board: Vec<Vec<String>>) {
    println!(" {} | {} | {}",board[0][0], board[0][1], board[0][2]);
    println!("---+---+---");
    println!(" {} | {} | {}",board[1][0], board[1][1], board[1][2]);
    println!("---+---+---");
    println!(" {} | {} | {}",board[2][0], board[2][1], board[2][2]);
}

fn is_winner(board: Vec<Vec<String>>) -> bool {
    // this function checks to see if there is a winner
    let mut winner: bool = true;
    for row in 0..3 {
        if (board[row][0]==board[row][1])&&(board[row][1]==board[row][2])&&(board[row][1]!=" ".to_string()) {
            winner=false;
            println!("player {}, you won",board[row][0]);

        }
    }

    for colomn in 0..3 {
        if (board[0][colomn]==board[1][colomn])&&(board[1][colomn]==board[2][colomn])&&(board[1][colomn]!=" ".to_string()) {
            winner=false;
            println!("player {}, you won",board[0][colomn]);
        }
    }
    
    let diag = 0;
    if (board[diag][diag]==board[diag+1][diag+1])&&(board[diag+1][diag+1]==board[diag+2][diag+2])&&(board[1][1]!=" ".to_string()) {
        winner=false;
        println!("player {}, you won",board[diag+1][diag+1]);
    }
    if (board[diag][diag+2]==board[diag+1][diag+1])&&(board[diag+1][diag+1]==board[diag+2][diag])&&(board[1][1]!=" ".to_string()) {
        winner=false;
        println!("player {}, you won",board[diag+1][diag+1]);
    }
    if !winner {
        print_pretty(board);
    }
    return winner;
}

fn is_full (mut board: Vec<Vec<String>>, symbol_1: String, symbol_2: String) {
    let mut count: i32 = 1;
    let mut winner: bool = true;

    // this function is to check if the board is full
    while count < 10 && winner {
        print_pretty(board.clone());
        board = start_gamming(board.clone(), symbol_1.clone(), symbol_2.clone(), count);

        if count == 9 {
            println!("The board is full. Game over.");
            if winner {
                println!("The game is a tie.");
            }
        }

        winner = is_winner(board.clone());

        count += 1;
    };
    if !winner {
        println!("game over");
    }

}

fn tic_tac_toe_main () {
    intro();
    let board: Vec<Vec<String>>= create_board();
    let (symbol_1, symbol_2) = sym();
    info!("player 1 is {}, player 2 is {}.", symbol_1.to_string(), symbol_2.to_string());
    is_full(board, symbol_1, symbol_2);
}

fn main() {
    logger::main(); // start logger

    tic_tac_toe_main();
    return;
}
