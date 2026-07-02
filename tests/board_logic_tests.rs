use sea_battle;
#[cfg(test)]
// --- Тесты для Cell (арифметика) ---
#[test]
fn test_cell_addition() {
    let a = sea_battle::Cell::new(1, 2);
    let b = sea_battle::Cell::new(3, 4);
    let c = a + b;
    assert_eq!(c, sea_battle::Cell::new(4, 6));
}

#[test]
fn test_cell_subtraction() {
    let a = sea_battle::Cell::new(5, 7);
    let b = sea_battle::Cell::new(2, 3);
    let c = a - b;
    assert_eq!(c, sea_battle::Cell::new(3, 4));
}

#[test]
fn test_cell_negative_result() {
    // Rust позволяет отрицательные i32, это нормально для вектора
    let a = sea_battle::Cell::new(2, 3);
    let b = sea_battle::Cell::new(5, 10);
    let c = a - b;
    assert_eq!(c, sea_battle::Cell::new(-3, -7));
}

// --- Тесты для Ship (логика корабля) ---

#[test]
fn test_ship_cells_horizontal() {
    let ship = sea_battle::Ship::new(sea_battle::Cell::new(0, 0), 3, true); // горизонтальный, длина 3
    let cells = ship.cells();
    assert_eq!(
        cells,
        vec![
            sea_battle::Cell::new(0, 0),
            sea_battle::Cell::new(0, 1),
            sea_battle::Cell::new(0, 2),
        ]
    );
}

#[test]
fn test_ship_cells_vertical() {
    let ship = sea_battle::Ship::new(sea_battle::Cell::new(2, 5), 4, false); // вертикальный, длина 4
    let cells = ship.cells();
    assert_eq!(
        cells,
        vec![
            sea_battle::Cell::new(2, 5),
            sea_battle::Cell::new(3, 5),
            sea_battle::Cell::new(4, 5),
            sea_battle::Cell::new(5, 5),
        ]
    );
}

#[test]
fn test_ship_hit_true() {
    let ship = sea_battle::Ship::new(sea_battle::Cell::new(10, 10), 2, true);
    assert!(ship.hit(&sea_battle::Cell::new(10, 10)));
    assert!(ship.hit(&sea_battle::Cell::new(10, 11)));
}

#[test]
fn test_ship_hit_false() {
    let ship = sea_battle::Ship::new(sea_battle::Cell::new(10, 10), 2, true);
    // Рядом, но не на корабле
    assert!(!ship.hit(&sea_battle::Cell::new(10, 9)));
    assert!(!ship.hit(&sea_battle::Cell::new(11, 10)));
    // Далеко
    assert!(!ship.hit(&sea_battle::Cell::new(0, 0)));
}

// --- Тесты для BoardError (корректное отображение) ---

#[test]
fn test_board_error_display() {
    assert_eq!(
        sea_battle::BoardError::Out.to_string(),
        "Выстрел за пределы поля!"
    );
    assert_eq!(
        sea_battle::BoardError::Used.to_string(),
        "Вы сюда уже стреляли!"
    );
    assert_eq!(
        sea_battle::BoardError::WrongShip.to_string(),
        "Не удалось разместить корабль"
    );
}

// --- Тесты для ShotResult и Shot (структуры) ---
#[test]
fn test_shot_creation() {
    let shot = sea_battle::Shot {
        coordinates: sea_battle::Cell::new(3, 7),
        result: sea_battle::ShotResult::Hit,
    };
    assert_eq!(shot.coordinates, sea_battle::Cell::new(3, 7));
    assert_eq!(shot.result, sea_battle::ShotResult::Hit);
}
