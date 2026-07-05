use rand::prelude::*;
use sea_battle::{Board, Cell, Game, Ship, ShotResult};

#[test]
fn ai_move_uses_prev_hit_when_available() {
    let mut game = Game::new(5, &[1]);
    game.user_board.ships.clear();
    game.user_board.busy.clear();
    game.user_board
        .field
        .fill(vec![sea_battle::SeaField::Sea; 5]);

    // Устанавливаем prev_hit в (2,2)
    game.user_board.prev_hit = sea_battle::Shot {
        coordinates: Cell::new(2, 2),
        result: ShotResult::Hit,
    };
    game.user_board.first_hit = game.user_board.prev_hit.clone();

    // Получаем список возможных клеток из креста вокруг (2,2)
    let candidates = game.user_board.get_free_cross(Cell::new(2, 2));

    // Берём первую кандидатскую клетку (если есть) и ставим туда корабль
    if let Some(&c) = candidates.first() {
        if !game.user_board.out(&c) {
            game.user_board.add_ship(Ship::new(c, 1, true)).expect(
                "Failed to add ship at first candidate in ai_move_uses_prev_hit_when_available",
            );
        }
    } else {
        panic!("get_free_cross((2,2)) returned empty list — no target for AI");
    }
    game.user_board.begin();

    let mut rng = SmallRng::from_seed([0u8; 32]);
    let hit = game.ai_move(&mut rng);

    assert!(
        hit,
        "ai_move did not hit first candidate from get_free_cross((2,2)): {:?}",
        candidates
    );
}

#[test]
fn ai_move_falls_back_to_first_hit_when_prev_hit_blocked() {
    let mut game = Game::new(5, &[1]);
    game.user_board.ships.clear();
    game.user_board.busy.clear();
    game.user_board
        .field
        .fill(vec![sea_battle::SeaField::Sea; 5]);

    // prev_hit = (2,2), first_hit = (1,1)
    game.user_board.prev_hit = sea_battle::Shot {
        coordinates: Cell::new(2, 2),
        result: ShotResult::Hit,
    };
    game.user_board.first_hit = sea_battle::Shot {
        coordinates: Cell::new(1, 1),
        result: ShotResult::Hit,
    };

    // Занимаем всё вокруг (2,2) и (1,1), чтобы обе ветки вернули пустой список
    for c in &[
        Cell::new(2, 1),
        Cell::new(2, 3),
        Cell::new(1, 2),
        Cell::new(3, 2),
        Cell::new(1, 0),
        Cell::new(0, 1),
        Cell::new(2, 1),
        Cell::new(1, 2),
    ] {
        if !game.user_board.out(c) {
            game.user_board.busy.push(*c);
        }
    }

    // Получаем кандидатов из first_hit
    let candidates = game.user_board.get_free_cross(Cell::new(1, 1));

    // Берём первую кандидатскую клетку и ставим туда корабль
    if let Some(&c) = candidates.first() {
        if !game.user_board.out(&c) {
            game.user_board
                .add_ship(Ship::new(c, 1, true))
                .expect("Failed to add ship at first candidate in ai_move_falls_back_to_first_hit_when_prev_hit_blocked");
        }
    } else {
        panic!("get_free_cross((1,1)) returned empty list — no fallback target for AI");
    }
    game.user_board.begin();

    let mut rng = SmallRng::from_seed([1u8; 32]);
    let hit = game.ai_move(&mut rng);

    assert!(
        hit,
        "ai_move did not hit first candidate from get_free_cross((1,1)): {:?}",
        candidates
    );
}

#[test]
fn ai_move_uses_random_when_no_hits() {
    let mut game = Game::new(5, &[1]);
    game.user_board.ships.clear();
    game.user_board.busy.clear();
    game.user_board
        .field
        .fill(vec![sea_battle::SeaField::Sea; 5]);
    game.user_board.begin(); // без кораблей

    game.user_board.prev_hit.result = ShotResult::Miss;
    game.user_board.first_hit.result = ShotResult::Miss;

    let mut rng = SmallRng::from_seed([2u8; 32]);
    let _ = game.ai_move(&mut rng); // просто убеждаемся, что не паникует
}

#[test]
fn ai_move_returns_false_on_miss() {
    let mut game = Game::new(5, &[1]);
    game.user_board.ships.clear();
    game.user_board.busy.clear();
    game.user_board
        .field
        .fill(vec![sea_battle::SeaField::Sea; 5]);
    game.user_board.begin(); // без кораблей

    game.user_board.prev_hit.result = ShotResult::Miss;
    game.user_board.first_hit.result = ShotResult::Miss;

    let mut rng = SmallRng::from_seed([3u8; 32]);
    let hit = game.ai_move(&mut rng);

    assert!(!hit);
    assert_eq!(game.user_board.prev_hit.result, ShotResult::Miss);
}
