use std::fmt::{self};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeaField {
    Sea,
    Ship,
    Hit,
    Miss,
}

impl fmt::Display for SeaField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeaField::Sea => write!(f, "~"),
            SeaField::Ship => write!(f, "█"),
            SeaField::Hit => write!(f, "X"),
            SeaField::Miss => write!(f, "▪"),
        }
    }
}

// --------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    Out,
    Used,
    WrongShip,
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Если попытка разместить "за пределы поля", вернем строку
            BoardError::Out => write!(f, "За пределами поля!"),

            // Если клетка уже занята
            BoardError::Used => write!(f, "Клетка уже занята!"),

            // Если корабль нельзя поставить
            BoardError::WrongShip => write!(f, "Не удалось разместить корабль"),
        }
    }
}

// --------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Структура результата выстрела
pub enum ShotResult {
    /// Поражение корабля
    Hit,
    /// Уничтожение (последняя ячейка корабля)
    Dead,
    /// За пределами игрового поля
    Out,
    /// Промах в пределах игрового поля
    Miss,
    /// Попытка выстрела по уже обработанной(известной) клетке
    Used,
}

#[derive(Debug, Clone, Copy)]
/// Структура выстрел
pub struct Shot {
    pub coordinates: Cell,
    pub result: ShotResult,
}

// --------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
                    Cell::new(self.bow.x + i as i32, self.bow.y)
                } else {
                    Cell::new(self.bow.x, self.bow.y + i as i32)
                }
            })
            .collect()
    }
    /// Проверка на попадание
    pub fn hit(&self, cell: &Cell) -> bool {
        self.cells().contains(cell)
    }
}

// --------------------------------------------------------------------
/// Игровое поле
pub struct Board {
    /// hide - нужно ли скрывать размещение кораблей при печати?
    pub hide: bool,
    /// Размер поля
    pub size: usize,
    /// Количество потопленных кораблей
    pub count: usize,
    /// Текущее состояние поля
    pub field: Vec<Vec<SeaField>>,
    /// занятые ячейки (либо кораблём, либо выстрелом)
    pub busy: Vec<Cell>,
    /// список кораблей доски
    pub ships: Vec<Ship>,
    /// последнее успешное поражение корабля.
    pub prev_hit: Shot,
    /// первое успешное поражение корабля в серии.
    pub first_hit: Shot,
}

impl Board {
    pub fn new(hide: bool, size: usize) -> Self {
        Self {
            hide,
            size,
            count: 0,
            field: vec![vec![SeaField::Sea; size]; size],
            busy: Vec::new(),
            ships: Vec::new(),
            prev_hit: Shot {
                coordinates: Cell::new(-1, -1),
                result: ShotResult::Out,
            },
            first_hit: Shot {
                coordinates: Cell::new(-1, -1),
                result: ShotResult::Out,
            },
        }
    }

    /// Проверка на расположение ячейки за пределы поля
    pub fn out(&self, d: &Cell) -> bool {
        !(d.x >= 0 && d.x < self.size as i32 && d.y >= 0 && d.y < self.size as i32)
    }

    /// Выдаёт значение ячейки по координатам
    pub fn get_cell(&self, d: &Cell) -> SeaField {
        self.field[d.x as usize][d.y as usize]
    }

    /// Сформировать честный контур вокруг корабля (show - отображать контур, если true или только записать в "занятые" если false)
    pub fn ship_board(&mut self, ship: &Ship, show: bool) {
        let near = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 0),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]; // все направления ячеек вокруг текущей (сдвиги по диагонали и вертикали)
        for d in ship.cells() {
            // берём каждую ячейку корабля...
            for &(dx, dy) in &near {
                // ...проходимся в цикле по списку направлений...
                let cur = Cell::new(d.x + dx, d.y + dy); // ...сдвигаем исходную ячейку на dx и dy
                if !self.out(&cur) && !self.busy.contains(&cur) {
                    // если она не выходит за пределы доски и не занята...
                    if show {
                        // если отображать
                        self.field[cur.x as usize][cur.y as usize] = SeaField::Miss; // ... то ставим знак промаха в ячейку
                    }
                    self.busy.push(cur); // добавляем ячейку в список занятых
                }
            }
        }
    }

    /// Добавление корабля на поле
    pub fn add_ship(&mut self, ship: Ship) -> Result<(), BoardError> {
        if ship
            .cells()
            .iter()
            .any(|d| self.out(d) || self.busy.contains(d))
        // проверка каждой ячейки корабля, что она не выходит за границу и не занята.
        {
            return Err(BoardError::WrongShip); // Возврат ошибки в случае проблемы
        }
        // Если всё хорошо
        for d in ship.cells() {
            // Для каждой ячейки корабля
            self.field[d.x as usize][d.y as usize] = SeaField::Ship; // поставим в каждой ячейке палубу корабля
            self.busy.push(d); // и запишем ячейку в список занятых (ячейки расположения корабля)
        }
        self.ship_board(&ship, false); // обводим список собственных кораблей по контуру и только записываем окружающие ячейки в занятые, без отображения (параметр false)
        self.ships.push(ship); // добавляем список присутствующих на поле кораблей
        Ok(())
    }

    /// Провести выстрел по клетке и получить результат ShotResult
    pub fn shot(&mut self, d: Cell) -> ShotResult {
        if self.out(&d) {
            // выходит ли ячейка за границу?
            return ShotResult::Out; // если да, возвращаем результат "за пределами"
        }
        if self.busy.contains(&d) {
            // занята ли ячейка?
            return ShotResult::Used; // сли да, возвращаем результат "уже использовано"
        }
        self.busy.push(d); // добавляем ячейку в занято (если не была занята)
        // Проходимся в цикле по кораблям и проверяем, принадлежит ли ячейка какому-либо кораблю или нет и если принадлежит, то:
        if let Some(i) = self.ships.iter().position(|s| s.hit(&d)) {
            self.ships[i].lives -= 1; // уменьшаем количество жизней корабля
            self.field[d.x as usize][d.y as usize] = SeaField::Hit; // маркируем соответствующим образом ячейку
            if self.ships[i].lives == 0 {
                // если у корабля кончились жизни, то
                self.count += 1; // прибавляем к счётчику уничтоженных кораблей единицу
                self.ship_board(&self.ships[i].clone(), true); //  обводим корабль, чтобы контур обозначился ячейками
                self.prev_hit = Shot {
                    coordinates: d,
                    result: ShotResult::Dead,
                };
                self.first_hit.result = ShotResult::Dead;
                return ShotResult::Dead;
            }
            if !matches!(self.prev_hit.result, ShotResult::Hit) {
                self.first_hit = Shot {
                    coordinates: d,
                    result: ShotResult::Hit,
                };
            }
            self.prev_hit = Shot {
                coordinates: d,
                result: ShotResult::Hit,
            };
            return ShotResult::Hit;
        }
        // если никакой корабль не поражён, срабатывает этот код, означающий промах
        self.field[d.x as usize][d.y as usize] = SeaField::Miss;
        ShotResult::Miss
    }

    /// обнуление списка использованных ячеек игры (где сохраняться выстрелы игрока и обведённые после уничтожения корабли).
    pub fn begin(&mut self) {
        self.busy.clear();
    }

    /// Проверка на поражение (если не осталось живых кораблей)
    pub fn defeat(&self) -> bool {
        // Победа только если есть корабли И все они уничтожены
        !self.ships.is_empty() && self.count == self.ships.len()
    }

    /// Получение списка возможных ячеек для следующего выстрела по переданным координатам (для AI)
    pub fn get_free_cross(&self, d: Cell) -> Vec<Cell> {
        let near = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut result = Vec::new();
        for &(dx, dy) in &near {
            let cur = Cell::new(d.x + dx, d.y + dy);
            if !self.out(&cur) && self.busy.contains(&cur) && self.get_cell(&cur) == SeaField::Hit {
                let cell = d + (d - cur);
                if !self.out(&cell) && !self.busy.contains(&cell) {
                    return vec![cell];
                }
            }
            if !self.out(&cur) && !self.busy.contains(&cur) {
                result.push(cur);
            }
        }
        result
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("  "); // два пробела как отступ
        for i in 1..=self.size {
            res.push_str(&i.to_string());
            res.push(' '); // пробел после каждого числа
        }
        res.push('\n');
        for (i, row) in self.field.iter().enumerate() {
            res.push_str(&format!("{} ", i + 1));
            for &cell in row {
                if self.hide && cell == SeaField::Ship {
                    res.push_str(&format!("{} ", SeaField::Sea.to_string()));
                } else {
                    res.push_str(&cell.to_string());
                    res.push(' ');
                }
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}
