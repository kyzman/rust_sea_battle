mod board;
mod player;

const FIELD_SIZE: i8 = 8;
// const FIELD_SIZE: i8 = 6;
const SHIPS: [u8; 10] = [4, 3, 3, 2, 2, 2, 1, 1, 1, 1];
// const SHIPS: [u8; 4] = [4, 3, 2, 1];

fn main() {
    println!(
        "Тут будет морской бой!\nРазмер поля: {}x{}\nКораблей: {}",
        FIELD_SIZE,
        FIELD_SIZE,
        SHIPS.len()
    );

    println!("Тест ячейки 1: {:?}", board::Cell::new(3, 4));
    println!(
        "Test cell 2: {:?}",
        board::Cell::new(5, 3) - board::Cell::new(2, 1)
    );
    println!(
        "Test cell 3: {:?}",
        board::Cell::new(2, 3) + board::Cell::new(3, 3)
    );
    println!(
        "Test shot 1: {:?}",
        board::Shot {
            coordinates: board::Cell::new(-1, -1),
            result: board::ShotResult::Out
        }
    );
    println!(
        "Test shot 2: {:?}",
        board::Shot {
            coordinates: board::Cell::new(3, 4),
            result: board::ShotResult::Hit
        }
    );
    let test_ship = board::Ship::new(board::Cell::new(3, 4), 6, true);
    println!("Test ship 1: {:?}", test_ship);
    println!("Test ship 2: {:?}", test_ship.cells());
    let test_cell = board::Cell::new(3, 5);
    println!("Test hit ship 1: {:?}", test_ship.hit(&test_cell));
    let test_cell = board::Cell::new(4, 5);
    println!("Test hit ship 2: {:?}", test_ship.hit(&test_cell));

    let mut game = player::Game::new(FIELD_SIZE as usize, &SHIPS);
    game.loop_game();
}
