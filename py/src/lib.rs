use ascacou_rs;

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[pyclass]
struct Board(ascacou_rs::Board);

#[pymethods]
impl Board {
    #[new]
    fn new(fen: &str) -> PyResult<Self> {
        match ascacou_rs::Board::from_fen(fen) {
            Ok(board) => Ok(Board(board)),
            Err(s) => Err(PyRuntimeError::new_err(s))
        }
    }

    fn is_terminal(&self) -> bool {
        self.0.is_terminal()
    }

    fn is_winning(&self) -> bool {
        self.0.is_winning()
    }

    fn possible_moves(&self) -> Vec<String> {
        self.0.possible_moves().into_iter().map(|mov|mov.into()).collect()
    }

    fn next(&self, mov: String) -> PyResult<Self> {
        match ascacou_rs::Move::try_from(mov) {
            Ok(mov) => Ok(Board(self.0.next(mov))),
            Err(s) => Err(PyRuntimeError::new_err(s))
        }
    }

    fn score(&self) -> i8 {
        self.0.current_score()
    }

    fn __str__(&self) -> String {
        self.0.for_console()
    }

    fn __repr__(&self) -> String {
        format!("<Board fen=\"{}\" score={}>", self.0.fen(), self.0.current_score())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn ascacou(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Board>()?;
    Ok(())
}
