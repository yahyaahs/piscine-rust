
pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('X',table) || horizontal('X', table) || vertical('X', table){
        return format!("player {} won", "X")
    }else if diagonals('O',table) || horizontal('O', table) || vertical('O', table){
                return format!("player {} won", "O")
    }
    return "tie".to_string();
;
}
pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0]== player &&table[0][0] == table[1][1]&& table[1][1]== table[2][2]{
        if table[0][0] == player{
            return true;

        }
    }
    if table[0][2]== player&&table[0][2] == table[1][1]&& table[1][1]== table[2][0]{
        if table[0][2] == player{
            return true;

        }    
    }
    return false;

}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3{
        if table[i][0] ==  table[i][1] && table[i][1]== table[i][2]{
            if  table[i][0] ==  player {
                return true;

            }
        }
    }
    
    return false;
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    for i in 0..3{
        if table[0][i] == table[1][i] && table[1][i]==table[2][i]{
            if table[0][i] == player{
                return true;

            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
            println!(
        "{}",
        tic_tac_toe([['O', 'X', 'O'], ['O', 'P', 'X'], ['X', '#', 'X']])
    );
    // tie
    println!(
        "{}",
        tic_tac_toe([['X', 'O', 'O'], ['X', 'O', 'O'], ['#', 'O', 'X']])
    );
    // player O won

    let diag = [['O', 'O', 'X'], ['O', 'X', 'O'], ['X', '#', 'X']];

    println!("{}", tic_tac_toe(diag));
    // player X won
    }
}
