use pyo3::prelude::*;
mod cli;

mod py_globals {
    use super::*;

    #[pyfunction]
    #[pyo3(signature = (physical_only=false))]
    pub fn num_cpus(physical_only: bool) -> usize {
        pyrust_internal::num_cpus_available(physical_only)
    }

    pub const BACKEND: &str = "Rust!";
}

fn register_submodule<'a>(
    name: &str,
    supermodule: &Bound<'a, PyModule>,
) -> PyResult<Bound<'a, PyModule>> {
    let submodule = PyModule::new(supermodule.py(), name)?;
    supermodule.add_submodule(&submodule)?;

    Ok(submodule)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _bindings(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let cli_module = register_submodule("cli", m)?;
    cli_module.add_class::<cli::PyCli>()?;

    m.add_function(wrap_pyfunction!(py_globals::num_cpus, m)?)?;
    m.add("backend", py_globals::BACKEND)?;

    Ok(())
}
