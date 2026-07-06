use rand::{RngExt, rng};
use sea_battle::{Board, Cell, Game, Ship, Shot, ShotResult};

#[test]
fn test_ai_gets_extra_turns_on_hit() {
    // Создаём игру с маленькой доской и одним длинным кораблём, чтобы контролировать попадания
    let ships = [4];
    let mut game = Game::new(5, &ships);

    // Вручную расставляем корабль игрока, чтобы знать, куда стрелять
    let mut user_board = Board::new(false, 5);
    let ship = Ship::new(Cell::new(1, 2), 4, true); // горизонтальный
    assert!(user_board.add_ship(ship).is_ok());
    user_board.begin();
    game.user_board = user_board;

    // Обнуляем статистику
    let mut rng = rng();
    game.user_board.busy.push(Cell::new(0, 2)); // добавляем клетку в "занято" чтобы он не пытался туда стрелять.
    let ai_shots_before = count_busy(&game.user_board);
    eprintln!("before: {}", ai_shots_before);

    // Заставляем ИИ попасть в первую клетку корабля (1,2)
    // Для этого подменяем логику выбора цели: временно ставим prev_hit/first_hit
    game.user_board.prev_hit = Shot {
        coordinates: Cell::new(0, 2),
        result: ShotResult::Hit, // для целей тестирования подменяем значение на "hit", не смотря на то, что там на самом деле нет корабля.
    };
    game.user_board.first_hit = game.user_board.prev_hit;

    // Теперь запускаем один «ход» ИИ — он должен сделать минимум 4 выстрела (по длине корабля)
    let mut ai_shot_count = 0;
    while game.ai_move(&mut rng) {
        ai_shot_count += 1;
        // Если корабль потоплен, флаг prev_hit должен сброситься, и цикл остановится
        if game.user_board.defeat() {
            break;
        }
    }

    let ai_shots_after = count_busy(&game.user_board);
    eprintln!("after: {}", ai_shots_after);
    let total_new_shots = ai_shots_after - ai_shots_before;

    // ИИ должен был сделать ровно 4 выстрела подряд (по количеству клеток корабля)
    assert_eq!(
        ai_shot_count, 4,
        "ИИ должен был попасть 4 раза подряд, пока не потопил корабль"
    );
    assert_eq!(
        total_new_shots,
        14, // Всего сигнатура корабля размером 4 равна 18 ( 4 сам корабль и 14 клетки вокруг),
        // однако тут 4 выстрела по кораблю от ИИ + 10 это обведение корабля после победы
        // (корабль примыкает к нижему краю поля, поэтому -3 клетки снизу и -1 была занята заранее, чтобы ИИ не стрелял вверх от носа).
        "На доске должно появиться ровно 14 занятых клеток"
    );
    assert!(game.user_board.defeat(), "Корабль должен быть потоплен");
}

#[test]
fn test_ai_uses_first_hit_after_miss() {
    let ships = [3];
    let mut game = Game::new(6, &ships);

    // Ставим корабль так, чтобы ИИ мог попасть в центр, промахнуться в одну сторону,
    // а затем продолжить с first_hit в другую сторону
    let mut user_board = Board::new(false, 6);
    let ship = Ship::new(Cell::new(2, 3), 3, false); // клетки: (2,3), (3,3), (4,3)
    assert!(user_board.add_ship(ship).is_ok());
    user_board.begin();
    game.user_board = user_board;

    let mut rng = rng();

    // Сценарий:
    // 1. ИИ попадает в (3,3) — это центр. first_hit и prev_hit = (3,3)
    game.user_board.prev_hit = Shot {
        coordinates: Cell::new(3, 3),
        result: ShotResult::Hit,
    };
    game.user_board.first_hit = game.user_board.prev_hit;

    // 2. ИИ стреляет в (4,3) — попадает. prev_hit = (4,3)
    // Подменяем логику: вручную делаем выстрел, чтобы продвинуться
    _ = game.user_board.shot(Cell::new(4, 3));

    // 3. Теперь от prev_hit (4,3) нет свободных соседей (корабль кончился вправо).
    // get_free_cross вернёт None.
    // В ai_move сработает ветка first_hit — он должен попробовать (2,3) (влево от first_hit)

    // Запускаем один выстрел ИИ — он должен выстрелить в (2,3)
    let shot = game.ai_move(&mut rng);
    // Проверяем, что выстрел был сделан именно в (2,3) и это попадание
    // Так как ai_move не возвращает координаты, проверяем busy и результат
    assert!(shot, "ИИ должен попасть в оставшуюся часть корабля");

    // Убеждаемся, что (2,3) теперь в busy
    assert!(game.user_board.busy.contains(&Cell::new(2, 3)));
}

// Хелпер для подсчёта выстрелов
fn count_busy(board: &Board) -> usize {
    board.busy.len()
}
