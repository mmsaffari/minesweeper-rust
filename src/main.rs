mod logic;
use logic::{CellState, Position};

fn main() {
    let pos = Position::new(2, 3);
    let cell_state = CellState::default();

    println!("{pos:?}");
    println!("{cell_state:?}");
}
