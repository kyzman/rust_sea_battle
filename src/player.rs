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
    fn try_board(size: usize, ships: &[u8]) -> Option<Board> {
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

    fn draw_fieds(&self, ai_step: usize, user_step: usize) {
        io::stdout().flush().unwrap();
        println!(
            "User[{}]:\n{}\nAI[{}]:\n{}",
            ai_step, self.user_board, user_step, self.ai_board
        );
    }

    pub fn loop_game(&mut self) {
        // Для rand 10+ используем rand::rng()
        let mut rng = rand::rng();
        let mut ai_step_num: usize = 0;
        let mut user_step_num: usize = 0;
        let mut ai_step_res: ShotResult;
        let mut user_step_res: ShotResult;

        self.draw_fieds(ai_step_num, user_step_num);

        loop {
            // --- ХОД ИГРОКА ---
            // Зацикливаем ход, пока он попадает или нет победы
            loop {
                user_step_res = self.user_move();
                if user_step_res != ShotResult::Out && user_step_res != ShotResult::Used {
                    user_step_num += 1
                }
                self.draw_fieds(ai_step_num, user_step_num);
                if user_step_res == ShotResult::Miss || self.ai_board.defeat() {
                    break;
                }
            }
            // Проверяем победу сразу после хода игрока
            if self.ai_board.defeat() {
                println!("User wins!");
                break;
            }

            // --- ХОД ИИ ---
            // Теперь нам нужно зациклить ход ИИ, пока он попадает или не победил
            loop {
                ai_step_res = self.ai_move(&mut rng);
                if ai_step_res != ShotResult::Out && ai_step_res != ShotResult::Used {
                    ai_step_num += 1
                }
                self.draw_fieds(ai_step_num, user_step_num);
                if ai_step_res == ShotResult::Miss || self.user_board.defeat() {
                    break;
                }
            }
            // Проверяем поражение игрока сразу после каждого выстрела ИИ
            if self.user_board.defeat() {
                self.draw_fieds(ai_step_num, user_step_num);
                println!("AI wins!");
                return; // Завершаем всю функцию game
            }
        }
    }

    pub fn user_move(&mut self) -> ShotResult {
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
            // Для корректного отображения пришлось x и y поменять местами,
            //  т.к. на самом деле у нас есть проблема в том, что в vec хранятся данные не правильно (x - это y, y - это x)
            return self.ai_board.shot(Cell::new(coords[1] - 1, coords[0] - 1));
        }
    }
    pub fn ai_move(&mut self, rng: &mut impl RngExt) -> ShotResult {
        // Сначала пытаемся найти цель от последнего попадания (prev_hit)
        let mut target: Option<Cell> = if matches!(self.user_board.prev_hit.result, ShotResult::Hit)
        {
            self.user_board
                .get_free_cross(self.user_board.prev_hit.coordinates)
                .into_iter()
                .next()
        } else {
            None
        };

        // Если от prev_hit стрелять некуда — пробуем от first_hit
        if target.is_none() && matches!(self.user_board.first_hit.result, ShotResult::Hit) {
            target = self
                .user_board
                .get_free_cross(self.user_board.first_hit.coordinates)
                .into_iter()
                .find(|c| !self.user_board.busy.contains(c));
        }

        // Если всё равно нет цели — случайный выстрел
        let shot = target.unwrap_or_else(|| {
            Cell::new(
                rng.random_range(0..self.ai_board.size as i32),
                rng.random_range(0..self.ai_board.size as i32),
            )
        });

        self.user_board.shot(shot)
    }
}
