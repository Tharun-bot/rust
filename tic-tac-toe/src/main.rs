use std::io;

fn main() {
    let mut board = [[0; 3]; 3];
    let mut count = 0;

    while count < 9 {
        input_and_check(&mut count, &mut board);

        let winner = check_winner(&board);
        if winner != 0 {
            println!("Player {} wins!", winner);
            println!("Final Board: {:?}", board);
            return;
        }
    }

    println!("It's a draw!");
    println!("Final Board: {:?}", board);
}

fn input_and_check(count: &mut i32, board: &mut [[i32; 3]; 3]) {
    let mut col = String::new();
    let mut row = String::new();

    loop {
        col.clear();
        row.clear();

        println!("Enter column (0-2):");
        io::stdin().read_line(&mut col).unwrap();
        let col_int: usize = match col.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        println!("Enter row (0-2):");
        io::stdin().read_line(&mut row).unwrap();
        let row_int: usize = match row.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        if row_int > 2 || col_int > 2 {
            println!("Out of bounds");
            continue;
        }

        if board[row_int][col_int] != 0 {
            println!("Place already filled, try again");
            continue;
        }

        if *count % 2 == 0 {
            board[row_int][col_int] = 1;
        } else {
            board[row_int][col_int] = 2;
        }

        *count += 1;
        break;
    }
}

fn check_winner(board: &[[i32; 3]; 3]) -> i32 {
    // Check for row equality
    for i in 0..3 {
        if board[i][0] != 0 && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return board[i][0];
        }
    }
    // Check columns
    for j in 0..3 {
        if board[0][j] != 0 && board[0][j] == board[1][j] && board[1][j] == board[2][j] {
            return board[0][j];
        }
    }
    // Main diagonal
    if board[0][0] != 0 && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return board[0][0];
    }
    // Anti-diagonal
    if board[0][2] != 0 && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return board[0][2];
    }

    return 0;
}