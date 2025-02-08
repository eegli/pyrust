use pyo3::prelude::*;
use pyrust_cli::cli;

#[pyclass]
pub struct PyCli {
    args: Vec<String>,
}

#[pymethods]
impl PyCli {
    #[new]
    pub fn new(args: Vec<String>) -> Self {
        Self { args }
    }

    pub fn run(&self) {
        cli::run(Some(&self.args))
    }
}
