use chess::{BoardStatus, Game, GameResult, MoveGen};
use core::fmt;
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Copy, Clone)]
struct MyResult {
    result: Result,
    length: usize,
}

const DEFAULT_MYRESULT: MyResult = MyResult {
    result: Result::Draw,
    length: 0,
};

impl fmt::Display for MyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.result {
            Result::White => {
                write!(f, "white,{}", self.length)
            }
            Result::Black => {
                write!(f, "black,{}", self.length)
            }
            Result::Draw => {
                write!(f, "draw,{}", self.length)
            }
            Result::Stalemate => {
                write!(f, "stalemate,{}", self.length)
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Result {
    White,
    Black,
    Draw,
    Stalemate,
}

fn main() {
    let mut results = vec![DEFAULT_MYRESULT; 240000];

    // THIS ONE IS SLOWER
    //results.par_chunks_mut(1000).for_each(|slice| {
    //    for res in slice.iter_mut() {
    //        *res = gen_game();
    //    }
    //});

    results.par_iter_mut().for_each(|res| {
        *res = gen_game();
    });

    let mut whitemates = 0;
    let mut blackmates = 0;
    let mut draws = 0;
    let mut stalemates = 0;
    for r in results {
        match r.result {
            Result::White => whitemates += 1,
            Result::Black => blackmates += 1,
            Result::Draw => draws += 1,
            Result::Stalemate => stalemates += 1,
        }
    }
    println!("white: {}", whitemates);
    println!("black: {}", blackmates);
    println!("draws: {}", draws);
    println!("stale: {}", stalemates);
}

fn gen_game() -> MyResult {
    let mut game = Game::new();

    let mut rng = rand::thread_rng();

    loop {
        let mut moves = MoveGen::new_legal(&game.current_position());
        let num_moves = moves.len();
        let status = game.current_position().status();
        match status {
            BoardStatus::Ongoing => (),
            BoardStatus::Stalemate => {
                break;
            }
            BoardStatus::Checkmate => {
                break;
            }
        }
        let board = *game.current_position().combined();
        if board.popcnt() == 2 {
            game.offer_draw(game.side_to_move());
            game.accept_draw();
            break;
        }
        if game.actions().len() == 1000 {
            game.offer_draw(game.side_to_move());
            game.accept_draw();
            break;
        }
        let my_move = rng.gen_range(0..num_moves);
        let rand_move = moves.nth(my_move).expect("no valid move");
        game.make_move(rand_move);
        //println!("{} ", rand_move);
    }
    let result = game.result().expect("no result");
    let game_length = game.actions().len();
    match result {
        GameResult::WhiteCheckmates => MyResult {
            result: Result::White,
            length: game_length,
        },
        GameResult::WhiteResigns => MyResult {
            result: Result::Black,
            length: game_length,
        },
        GameResult::BlackCheckmates => MyResult {
            result: Result::Black,
            length: game_length,
        },
        GameResult::BlackResigns => MyResult {
            result: Result::White,
            length: game_length,
        },
        GameResult::Stalemate => MyResult {
            result: Result::Stalemate,
            length: game_length,
        },
        GameResult::DrawAccepted => MyResult {
            result: Result::Draw,
            length: game_length,
        },
        GameResult::DrawDeclared => MyResult {
            result: Result::Draw,
            length: game_length,
        },
    }
}
