pub mod utils;

use crate::utils::game::Game;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn fency_pgn(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fentasize, m)?)?;
    Ok(())
}

#[pyfunction]
fn fentasize(moves: Vec<&str>) -> PyResult<Vec<String>> {
    let mut game = Game::new();
    let fens: Vec<String> = moves.iter().fold(Vec::new(), |mut acc, &mv| {
        game.play_move(mv);
        acc.push(game.clone().to_fen());
        acc
    });

    Ok(fens)
}
