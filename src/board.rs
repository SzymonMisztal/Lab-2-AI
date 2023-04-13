pub struct Board {}

impl Board {
    pub fn reset_board(board: &mut [[char; 8]; 8]) {
        for y in board.iter_mut() {
            for x in y.iter_mut() {
                *x = ' ';
            }
        }
        board[3][3] = 'W';
        board[4][4] = 'W';
        board[3][4] = 'B';
        board[4][3] = 'B';
    }

    pub fn create_board() -> [[char; 8]; 8] {
        let mut board = [[' '; 8]; 8];
        board[3][3] = 'W';
        board[4][4] = 'W';
        board[3][4] = 'B';
        board[4][3] = 'B';
        return board;
    }

    pub fn print_board(board: &[[char; 8]; 8]) {
        println!("╔═══╦═══╦═══╦═══╦═══╦═══╦═══╦═══╗");
        for (i, y) in board.iter().enumerate() {
            for x in y.iter() {
                print!("║ {} ", x);
            }
            println!("║");
            if i != board.len() - 1 {
                println!("╠═══╬═══╬═══╬═══╬═══╬═══╬═══╬═══╣");
            }
        }
        println!("╚═══╩═══╩═══╩═══╩═══╩═══╩═══╩═══╝");
    }
}