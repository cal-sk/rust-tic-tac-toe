use ::std::*;

fn main() {
    // define rows
    let mut row1: [&str; 3] = ["", "", ""];
    let mut row2: [&str; 3] = ["", "", ""];
    let mut row3: [&str; 3] = ["", "", ""];
    let mut win = false;
    let mut draw = false;

    let row_names1 = "A";
    let row_names2 = "B";
    let row_names3 = "C";
    let column_names = "1   2   3";

    // turn logic
    let mut player_turn = "X";
    let player2_turn = "O";
    let empty: &str = "";
    let running = true;

    // game loop
    while running {

        // checks if it is player1 turn
        if  win == false && draw == false {

            // empty moves variables when if statement starts
            let mut move_x = String::new();
            let mut move_y = String::new();
            let real_move_x: usize;

            // check turn
            if player_turn == player2_turn {
                println!("Player 2 Turn")
            } 
            else {
                println!("Player 1 Turn");
            }

            // print each row 
            print!("    {}", row_names1);
            println!("{:?}", row1);

            print!("    {}", row_names2);
            println!("{:?}", row2);

            print!("    {}", row_names3);
            println!("{:?}", row3);
            println!("      {}", column_names);

            // column input
            println!("Input which column to place your move");
            std::io::stdin().read_line(&mut move_x).expect("Erro");
            real_move_x = move_x.trim_end().parse().expect("err");

            // row input
            println!("Input which row to place your move");
            std::io::stdin().read_line(&mut move_y).expect("Erro");
            move_y = move_y.trim_end().parse().expect("err");


            // row b logic
            if move_y == "B".to_string() || move_y == "b".to_string() {
                // check if spot is empty if it is spot already taken
                if row2[real_move_x-1] != empty {
                    println!("Spot already taken");
                }

                else {
                    row2[real_move_x-1] = player_turn;
                    // check which turn it is and change turns 
                    if player_turn == "X" {
                        player_turn = "O";
                    }
                    else if player_turn == "O" {
                       player_turn = "X";
                    }
                }
            }

            // row a logic
            else if move_y == "A".to_string() || move_y == "a".to_string() {
                // check if spot is empty if it is spot already taken
                if row1[real_move_x-1] != empty{
                    println!("Spot already taken");
                }

                else {
                    row1[real_move_x-1] = player_turn;
                    println!("{:?}", row1[real_move_x-1]);
                    // check which turn it is and change turns 
                    if player_turn == "X" {
                        player_turn = "O";
                    }
                    else if player_turn == "O" {
                        player_turn = "X";
                    }
                }

            }

            // row c logic
            else if move_y == "C".to_string() || move_y == "c".to_string() {
                // check if spot is empty if it is spot already taken
                if row3[real_move_x-1] != empty {
                    println!("Spot already taken");
                    
                }

                else {
                    row3[real_move_x-1] = player_turn;
                    // check which turn it is and change turns 
                    if player_turn == "X" {
                        player_turn = "O";
                    }
                    else if player_turn == "O" {
                        player_turn = "X";
                    }
                }

            }
            // print that it is invalid input
            else {
                println!("invalid input try again")
            }
            if check_win(&mut row1, &mut row2, &mut row3) == true {
                if player_turn == "X" {
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                    println!("x won!!");
                } else if player_turn == "O" {
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                    println!("o won!!");
                }
                win = true;
            } 
            else {
                if check_draw(&mut row1, &mut row2, &mut row3) == true {
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    println!("a draw!");
                    draw = true;
                }
            }

            // create win condition logic

        } // temporary else break
        else {
            break;
        }
    }    
}
fn check_draw(row_a: &mut [&str; 3], row_b: &mut [&str; 3], row_c: &mut [&str; 3])-> bool {
    if row_a[0] != "" && row_a[1] != "" && row_a[2] != "" {
        if row_b[0] != "" && row_b[1] != "" && row_b[2] != "" {
            if row_c[0] != "" && row_c[1] != "" && row_c[2] != "" {
                return true;
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
    }
    else {
        return false;
    }
}

fn check_win(row_a: &mut [&str; 3], row_b: &mut [&str; 3], row_c: &mut [&str; 3])-> bool {

    // up and down check
    if row_a[1] == row_b[1] && row_b[1] == row_c[1] && row_a[1] != ""{
        return true;
    } 
    else if  row_a[2] == row_b[2] && row_b[2] == row_c[2] &&row_a[2] != "" {
        return true;
    }
    else if  row_a[0] == row_b[0] && row_b[0] == row_c[0] && row_a[0] != ""{
        return true;
    }
    // diag check
    else if row_a[0] == row_b[1] && row_b[1] == row_c[2] && row_a[0] != ""{
        return true;
    } 
    else if  row_a[2] == row_b[1] && row_b[1] == row_c[0] &&row_a[2] != "" {
        return true;
    }
    // horizontal check
    else if row_a[0] == row_a[1] && row_a[1] == row_a[2] && row_a[0] != "" {
        return true;
    }
    else if row_b[0] == row_b[1] && row_b[1] == row_b[2] && row_b[0] != "" {
        return true;
    }
    else if row_c[0] == row_c[1] && row_c[1] == row_c[2] && row_c[0] != "" {
        return true;
    }
    // if non return false
    else {
        return false;
    }
}