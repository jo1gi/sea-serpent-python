use pyo3::prelude::*;

#[pyclass]
struct Database(seaserpent::database::Database);

#[pymodule]
fn sea_serpent(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Database>()?;
    Ok(())
}
