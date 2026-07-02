use std::fmt;
use std::ops::{Add, Sub};
// --------------------------------------------------------------------

#[derive(Debug)]
pub enum BoardError {
    Out,
    Used,
    WrongShip,
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Если сработало "Выстрел за пределы", вернем строку
            BoardError::Out => write!(f, "Выстрел за пределы поля!"),

            // Если клетка уже занята
            BoardError::Used => write!(f, "Вы сюда уже стреляли!"),

            // Если корабль нельзя поставить
            BoardError::WrongShip => write!(f, "Не удалось разместить корабль"),
        }
    }
}

// --------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
/// Структура поля ячейки
pub struct Cell {
    x: i32,
    y: i32,
}

impl Cell {
    /// Создание ячейки
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Sub for Cell {
    type Output = Self;
    /// Разница ячеек
    fn sub(self, other: Self) -> Self::Output {
        Cell::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Cell {
    type Output = Self;
    /// Сумма ячеек
    fn add(self, other: Self) -> Self::Output {
        Cell::new(self.x + other.x, self.y + other.y)
    }
}

// --------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
/// Структура результата выстрела
pub enum ShotResult {
    Hit,
    Dead,
    Out,
}

#[derive(Debug)]
/// Структура выстрел
pub struct Shot {
    pub coordinates: Cell,
    pub result: ShotResult,
}

// --------------------------------------------------------------------
#[derive(Debug)]
/// Структура Корабль
pub struct Ship {
    /// ячейка носа корабля
    bow: Cell,
    /// длина корабля
    length: usize,
    /// ориентация корабля (False - вертикальный, True - горизонтальный)
    horizontal: bool,
    /// жизнь корабля измеряется его длинной
    lives: usize,
}

impl Ship {
    pub fn new(bow: Cell, length: usize, horizontal: bool) -> Self {
        Self {
            bow,
            length,
            horizontal,
            lives: length,
        }
    }
    /// Список координат всех ячеек корабля
    pub fn cells(&self) -> Vec<Cell> {
        (0..self.length)
            .map(|i| {
                if self.horizontal {
                    Cell::new(self.bow.x, self.bow.y + i as i32)
                } else {
                    Cell::new(self.bow.x + i as i32, self.bow.y)
                }
            })
            .collect()
    }
    /// Проверка на попадание
    pub fn hit(&self, cell: &Cell) -> bool {
        self.cells().contains(cell)
    }
}
