use crate::board::{Board, Cell, Ship, ShotResult};
use rand::RngExt;
use std::io::{self, Write};

pub struct Game {
    pub user_board: Board,
    pub ai_board: Board,
}

impl Game {
    pub fn new(size: usize, ships: &[u8]) -> Self {
        let user_board = Self::random_board(size, ships);
        let mut ai_board = Self::random_board(size, ships);
        ai_board.hide = true;
        Self {
            user_board,
            ai_board,
        }
    }
    pub fn try_board(size: usize, ships: &[u8]) -> Option<Board> {
        let mut board = Board::new(false, size);
        let mut rng = rand::rng();
        for ship_size in ships {
            let mut attempts = 0;
            while attempts < 100 {
                let ship = Ship::new(
                    Cell::new(
                        rng.random_range(0..size as i32),
                        rng.random_range(0..size as i32),
                    ),
                    *ship_size as usize,
                    rand::random(),
                );
                if board.add_ship(ship).is_ok() {
                    break;
                }
                attempts += 1;
            }
            if attempts == 100 {
                return None;
            }
        }
        board.begin();
        Some(board)
    }
    pub fn random_board(size: usize, ships: &[u8]) -> Board {
        // TODO: Потенциальная проблема бесконечного цикла. Надо сделать ограниченный цикл и выводить ошибку, если "не смогла"
        loop {
            if let Some(b) = Self::try_board(size, ships) {
                return b;
            }
        }
    }
    pub fn loop_game(&mut self) {
        // Для rand 10+ используем rand::rng()
        let mut rng = rand::rng();
        let mut ai_step: usize = 0;
        let mut user_step: usize = 0;

        loop {
            println!(
                "User[{}]:\n{}\nAI[{}]:\n{}",
                user_step, self.user_board, ai_step, self.ai_board
            );

            // --- ХОД ИГРОКА ---
            while self.user_move() || self.ai_board.defeat() {
                user_step += 1;
                io::stdout().flush().unwrap();
                println!(
                    "User[{}]:\n{}\nAI[{}]:\n{}",
                    user_step, self.user_board, ai_step, self.ai_board
                );
            }
            // Проверяем победу сразу после хода игрока
            if self.ai_board.defeat() {
                println!("User wins!");
                break;
            }

            // --- ХОД ИИ ---
            // Теперь нам нужно зациклить ход ИИ, пока он попадает
            while self.ai_move(&mut rng) || self.user_board.defeat() {
                ai_step += 1;
                io::stdout().flush().unwrap();
                println!(
                    "User[{}]:\n{}\nAI[{}]:\n{}",
                    user_step, self.user_board, ai_step, self.ai_board
                );
            }
            // Проверяем поражение игрока сразу после каждого выстрела ИИ
            if self.user_board.defeat() {
                println!("AI wins!");
                return; // Завершаем всю функцию game
            }

            // Опционально: можно добавить вывод сообщения, чтобы видеть, что ИИ ходит повторно
            // println!("AI hit again! Shooting again...");
        }
    }

    pub fn user_move(&mut self) -> bool {
        loop {
            print!("Shot (x y): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let coords: Vec<i32> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if coords.len() != 2 {
                continue;
            }
            match self.ai_board.shot(Cell::new(coords[0] - 1, coords[1] - 1)) {
                ShotResult::Miss => return false,
                _ => return true,
            }
        }
    }
    pub fn ai_move(&mut self, rng: &mut impl RngExt) -> bool {
        let d = if matches!(self.user_board.prev_hit.result, ShotResult::Hit) {
            self.user_board
                .get_free_cross(self.user_board.prev_hit.coordinates)
                .into_iter()
                .next()
                .unwrap_or(Cell::new(
                    rng.random_range(0..self.ai_board.size as i32),
                    rng.random_range(0..self.ai_board.size as i32),
                ))
        } else {
            Cell::new(
                rng.random_range(0..self.ai_board.size as i32),
                rng.random_range(0..self.ai_board.size as i32),
            )
        };
        eprintln!("{:?}", d);
        match self.user_board.shot(d) {
            ShotResult::Miss => return false,
            _ => return true,
        }
    }
}
