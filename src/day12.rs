use std::borrow::Borrow;
use std::sync::Arc;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

const WALL: char = '⬜';
const EMPTY: char = '⬛';
const COOKIE: char = '🍪';
const MILK: char = '🥛';

#[derive(Serialize,Deserialize,Clone,Copy,PartialEq,Default)]
#[serde(rename_all = "snake_case")]
pub enum BoardValue {
    #[default]
    Empty,
    Cookie,
    Milk,
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
    pub grid: [[BoardValue;4];4],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[BoardValue::Empty;4];4]
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
}

#[get("/12/board")]
async fn board(data: web::Data<Arc<RwLock<Board>>>) -> impl Responder {
    let data = data.read().await;

    HttpResponse::Ok().body(data.build_to_string())
}

// /12/reset
#[post("/12/reset")]
async fn reset(data: web::Data<Arc<RwLock<Board>>>) -> impl Responder {
    let mut data = data.write().await;
    *data = Board::default();

    HttpResponse::Ok().body(data.build_to_string())
}