#[derive(Eq, PartialEq, Clone, Copy)]
enum Players {
    VertPlayer,
    HorPlayer,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum BoardValues {
    Empty,
    Vertical,
    Horizontal,
    Cross
}

struct GameState {
    board: [[BoardValues; 3]; 3],
    current_turn: Players,
    last_played_pos: (u8, u8)
}

impl GameState {
    /*
    Función que indica si el estado del tablero es o no una posición ganadora
    */
    fn is_winning_pos(&self) -> bool {
        let b = &self.board;
        let c = BoardValues::Cross;
        (b[0][0] == c && b[0][1] == c && b[0][2] == c) || //horizontal wins
        (b[1][0] == c && b[1][1] == c && b[1][2] == c) ||
        (b[2][0] == c && b[2][1] == c && b[2][2] == c) ||
        (b[0][0] == c && b[1][0] == c && b[2][0] == c) || //vertical wins
        (b[0][1] == c && b[1][1] == c && b[2][1] == c) ||
        (b[0][2] == c && b[1][2] == c && b[2][2] == c) ||
        (b[0][0] == c && b[1][1] == c && b[2][2] == c) || //diagonal wins
        (b[0][2] == c && b[1][1] == c && b[2][0] == c)
    }
    
    /* 
    Función que retorna un iterador con todos los estados a los que se puede
    pasar en un movimiento desde el estado del tablero contenido en la instancia
    del struct
    */
    fn successor_states(&self) -> std::vec::IntoIter<GameState> {
        let mut accum: Vec<GameState> = vec![];
        let turn = self.current_turn;
        for (row_num, row) in self.board.iter().enumerate() {
            for (col_num, elem) in row.iter().enumerate() {
                if *elem != BoardValues::Cross && (row_num as u8, col_num as u8) != self.last_played_pos {
                    let mut new_game = GameState {
                        board: self.board.clone(),
                        current_turn: if turn == Players::VertPlayer { Players::HorPlayer } else { Players::VertPlayer },
                        last_played_pos: (row_num as u8, col_num as u8)
                    };
                    //Place new move in board
                    match elem {
                        BoardValues::Empty => {
                            new_game.board[row_num][col_num] = if turn == Players::VertPlayer { BoardValues::Vertical } else { BoardValues::Horizontal };
                        }
                        _ => {
                            new_game.board[row_num][col_num] = BoardValues::Cross
                        }
                    }
                    accum.push(new_game);
                }
            }
        }
        accum.into_iter()
    }
}


/*
Utiliza el método minimax para determinar si alguno de los jugadores de la versión
modificada de la vieja tiene una estrategia ganadora para un estado del tablero dado.
*/
fn minimax(game_state: &GameState, alfa: i32, beta: i32) -> i32 {
    if game_state.is_winning_pos() {
        //Para esta versión de la vieja no hay manera de empatar, así que el caso no se considera
        if game_state.current_turn == Players::VertPlayer {
            return -1 //Gana el jugador horizontal
        } else {
            return 1 //Gana el jugador vertical
        }
    }

    match game_state.current_turn {
        Players::VertPlayer => {
            let mut new_alpha = alfa;
            for action in game_state.successor_states() {
                let value = minimax(&action, new_alpha, beta);
                new_alpha = std::cmp::max(alfa, value);
                if new_alpha >= beta {
                    break;
                }
            }
            return new_alpha;
        }
        Players::HorPlayer => {
            let mut new_beta = beta;
            for action in game_state.successor_states() {
                let value = minimax(&action, alfa, new_beta);
                new_beta = std::cmp::min(new_beta, value);
                if new_beta <= alfa {
                    break;
                }
            }
            return new_beta;
        }
    }
}

fn main() {
    let initial_game_state = GameState {
        board: [[BoardValues::Empty; 3]; 3],
        current_turn: Players::VertPlayer,
        last_played_pos: (4, 4)
    };

    println!("Value for root node: {}", minimax(&initial_game_state, i32::MIN, i32::MAX));
}
