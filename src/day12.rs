use actix_web::{get, post, web, HttpResponse, Responder};
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::sync::Arc;
use tokio::sync::RwLock;

const WALL: char = '⬜';
const EMPTY: char = '⬛';
const COOKIE: char = '🍪';
const MILK: char = '🥛';

#[derive(Debug, Default, Clone, Copy, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BoardValue {
    Cookie,
    Milk,
    #[default]
    Empty,
}

impl BoardValue {
    pub fn convert_to_char(&self) -> char {
        match self {
            BoardValue::Empty => EMPTY,
            BoardValue::Cookie => COOKIE,
            BoardValue::Milk => MILK,
        }
    }
}

pub struct Board {
    pub grid: [[BoardValue; 4]; 4],
    pub winner: Option<BoardValue>,
    pub rng: rand::rngs::StdRng,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[BoardValue::Empty; 4]; 4],
            winner: None,
            rng: rand::rngs::StdRng::seed_from_u64(2024),
        }
    }
}

impl Board {
    pub fn build_to_string(&self) -> String {
        let mut state = [[WALL; 6]; 5];
        // Replacing the middle of the 6x5 grid, with the 4x4 gird
        for row in 0..self.grid.len() {
            for col in 0..self.grid.len() {
                state[row][col + 1] = self.grid[row][col].convert_to_char();
            }
        }

        state
            .into_iter()
            .map(|row| {
                // concert chars to string
                let mut res: String = row.into_iter().collect();
                res.push('\n');
                res
            })
            .collect()
    }

    pub fn get_current_state(&self) -> String {
        let mut state = self.build_to_string();
        println!("Winner {:?}", self.winner);

        if let Some(winner) = self.winner {
            if winner != BoardValue::Empty {
                state.push_str(&format!("{} wins!\n", winner.convert_to_char()));
            } else {
                state.push_str("No winner.\n");
            }
        }

        state
    }

    pub fn generate_random_board(&mut self) {
        let grid = &mut self.grid;
        self.winner = Some(BoardValue::Empty);

        // populating the grid
        for row in grid.iter_mut() {
            for val in row.iter_mut() {
                let res = self.rng.gen::<bool>();
                if res {
                    *val = BoardValue::Cookie
                } else {
                    *val = BoardValue::Milk
                }
            }
        }

        // check for winners
        // horizontal
        for y in 0..4 {
            if grid[y].iter().all(|&t| t == BoardValue::Cookie) {
                self.winner = Some(BoardValue::Cookie);
                return;
            } else if grid[y].iter().all(|&t| t == BoardValue::Milk) {
                self.winner = Some(BoardValue::Milk);
                return;
            }
        }

        // vertical
        for x in 0..4 {
            if (0..grid[0].len()).all(|y| grid[y][x] == BoardValue::Cookie) {
                self.winner = Some(BoardValue::Cookie);
                return;
            } else if (0..grid[0].len()).all(|y| grid[y][x] == BoardValue::Milk) {
                self.winner = Some(BoardValue::Milk);
                return;
            }
        }

        // tl -> br
        if (0..grid.len()).all(|i| grid[i][i] == BoardValue::Cookie) {
            self.winner = Some(BoardValue::Cookie);
            return;
        } else if (0..grid.len()).all(|i| grid[i][i] == BoardValue::Milk) {
            self.winner = Some(BoardValue::Milk);
            return;
        }

        // br -> tl
        if (0..grid.len()).all(|i| grid[grid.len() - i - 1][i] == BoardValue::Cookie) {
            self.winner = Some(BoardValue::Cookie);
        } else if (0..grid.len()).all(|i| grid[grid.len() - i - 1][i] == BoardValue::Milk) {
            self.winner = Some(BoardValue::Milk);
        }
    }
}

#[get("/12/board")]
async fn board(data: web::Data<Arc<RwLock<Board>>>) -> impl Responder {
    println!("board exec");
    let data = data.read().await;

    HttpResponse::Ok().body(data.get_current_state())
}

// /12/reset
#[post("/12/reset")]
async fn reset(data: web::Data<Arc<RwLock<Board>>>) -> impl Responder {
    println!("reset exec");
    let mut data = data.write().await;
    *data = Board::default();

    HttpResponse::Ok().body(data.get_current_state())
}

#[post("/12/place/{team}/{column}")]
pub async fn place(
    info: web::Path<(BoardValue, u8)>,
    data: web::Data<Arc<RwLock<Board>>>,
) -> HttpResponse {
    let (team, column) = info.into_inner();
    println!("input: {:?}", (team, column));
    let mut data = data.write().await;

    if !(1..=4).contains(&column) {
        // return HttpResponse::Ok().finish();
        return HttpResponse::BadRequest().finish();
    }
    let column = (column - 1) as usize;

    if data.winner.is_some() {
        return HttpResponse::ServiceUnavailable().body(data.get_current_state().to_string());
    }

    let Some(y) = data
        .grid
        .iter()
        .rev()
        .position(|row| row[column] == BoardValue::Empty)
    else {
        return HttpResponse::ServiceUnavailable().body(data.get_current_state().to_string());
    };
    let y = data.grid.len() - y - 1;

    data.grid[y][column] = team;
    println!("{}", data.get_current_state());

    // horizontal
    if data.grid[y].iter().all(|&t| t == team) {
        data.winner = Some(team);
    }

    // vertical
    if (0..data.grid[0].len()).all(|y| data.grid[y][column] == team) {
        data.winner = Some(team);
    }

    // tl -> br
    if (0..data.grid.len()).all(|i| data.grid[i][i] == team) {
        data.winner = Some(team);
    }

    // br -> tl
    if (0..data.grid.len()).all(|i| data.grid[data.grid.len() - i - 1][i] == team) {
        data.winner = Some(team);
    }

    // no winner
    if data
        .grid
        .iter()
        .all(|r| r.iter().all(|&t| t != BoardValue::Empty))
    {
        data.winner = Some(BoardValue::Empty);
    }

    // Check winning one more time
    HttpResponse::Ok().body(data.get_current_state().to_string())
}

#[get("/12/random-board")]
pub async fn random_board(data: web::Data<Arc<RwLock<Board>>>) -> HttpResponse {
    println!("random exec");
    let mut data = data.write().await;
    data.generate_random_board();

    HttpResponse::Ok().body(data.get_current_state().to_string())
}
