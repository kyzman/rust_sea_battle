use sea_battle::{Board, BoardError, Cell, SeaField, Ship, ShotResult};

#[test]
fn test_board_creation() {
    let board = Board::new(false, 10);
    assert_eq!(board.size, 10);
    assert!(!board.defeat());
    assert_eq!(board.count, 0);
}

#[test]
fn test_add_ship_success() {
    let mut board = Board::new(false, 10);
    let ship = Ship::new(Cell::new(0, 0), 3, true); // горизонтальный

    assert!(board.add_ship(ship).is_ok());
    // Корабль добавлен, значит, в списке кораблей он есть
    // (если у тебя нет публичного метода для доступа к ships, можно проверить косвенно)
}

#[test]
fn test_add_ship_out_of_bounds() {
    let mut board = Board::new(false, 5);
    // Пытаемся поставить корабль, который выходит за границы
    let ship = Ship::new(Cell::new(4, 0), 2, true);

    assert_eq!(board.add_ship(ship), Err(BoardError::WrongShip));
}

#[test]
fn test_add_ship_overlap() {
    let mut board = Board::new(false, 10);

    // Ставим первый корабль
    let ship1 = Ship::new(Cell::new(0, 0), 3, true);
    board.add_ship(ship1).unwrap();

    // Пытаемся поставить второй корабль на то же место
    let ship2 = Ship::new(Cell::new(0, 1), 2, true);

    assert_eq!(board.add_ship(ship2), Err(BoardError::WrongShip));
}

#[test]
fn test_shot_miss() {
    let mut board = Board::new(false, 10);
    let result = board.shot(Cell::new(5, 5));

    assert_eq!(result, ShotResult::Miss);
    // Проверяем, что клетка теперь помечена как промах (если есть доступ к полю)
    // Если поле приватное, этот тест можно опустить или добавить геттер для тестов
}

#[test]
fn test_shot_hit() {
    let mut board = Board::new(false, 10);
    let ship = Ship::new(Cell::new(2, 2), 3, false); // вертикальный
    board.add_ship(ship).unwrap();
    board.begin();
    let result = board.shot(Cell::new(2, 3)); // Попадание в середину корабля

    assert_eq!(result, ShotResult::Hit);
}

#[test]
fn test_shot_dead() {
    let mut board = Board::new(false, 10);
    let ship = Ship::new(Cell::new(5, 5), 1, true); // Одноклеточный корабль
    board.add_ship(ship).unwrap();
    board.begin();
    let result = board.shot(Cell::new(5, 5)); // Выстрел в корабль

    assert_eq!(result, ShotResult::Dead);
    assert!(board.defeat()); // Должен быть потоплен
}

#[test]
fn test_shot_out_of_bounds() {
    let mut board = Board::new(false, 10);
    let result = board.shot(Cell::new(-1, 5));

    assert_eq!(result, ShotResult::Out);
}

#[test]
fn test_shot_used() {
    let mut board = Board::new(false, 10);

    // Первый выстрел
    board.shot(Cell::new(1, 1));
    // Повторный выстрел в ту же клетку
    let result = board.shot(Cell::new(1, 1));

    assert_eq!(result, ShotResult::Used);
}

#[test]
fn test_ship_contour() {
    // Проверяем, что вокруг корабля появляются занятые клетки (контур)
    let mut board = Board::new(false, 10);
    let ship = Ship::new(Cell::new(5, 5), 1, true);
    board.add_ship(ship).unwrap();

    // Клетка (5,5) занята кораблём.
    // Клетки вокруг (4,4), (4,5), (4,6), (5,4), (5,6), (6,4), (6,5), (6,6) должны стать занятыми.
    let neighbors = [
        Cell::new(4, 4),
        Cell::new(4, 5),
        Cell::new(4, 6),
        Cell::new(5, 4),
        Cell::new(5, 6),
        Cell::new(6, 4),
        Cell::new(6, 5),
        Cell::new(6, 6),
    ];

    for n in neighbors.iter() {
        // Пытаемся выстрелить в соседнюю клетку — должна быть ошибка Used,
        // потому что она занята контуром
        let result = board.shot(*n);
        assert_eq!(
            result,
            ShotResult::Used,
            "Контур не был создан вокруг корабля!"
        );
    }
}

#[test]
fn test_defeat_condition() {
    let mut board = Board::new(false, 10);

    // Добавляем два корабля
    assert!(board.add_ship(Ship::new(Cell::new(0, 0), 1, true)).is_ok());
    assert!(board.add_ship(Ship::new(Cell::new(9, 9), 1, true)).is_ok());
    board.begin();
    assert_eq!(board.count, 0);
    assert_eq!(board.ships.len(), 2);

    assert!(!board.defeat());

    // Топим оба
    assert_eq!(board.shot(Cell::new(0, 0)), ShotResult::Dead);
    assert_eq!(board.count, 1);
    assert_eq!(board.ships.len(), 2);

    board.shot(Cell::new(9, 9));

    assert!(board.defeat());
}

#[test]
fn test_sea_field_display() {
    assert_eq!(SeaField::Sea.to_string(), "~");
    assert_eq!(SeaField::Ship.to_string(), "█");
    assert_eq!(SeaField::Hit.to_string(), "X");
    assert_eq!(SeaField::Miss.to_string(), "▪");
}

#[test]
fn test_cell_arithmetic() {
    let a = Cell::new(1, 2);
    let b = Cell::new(3, 4);

    assert_eq!(a + b, Cell::new(4, 6));
    assert_eq!(a - b, Cell::new(-2, -2));
}
