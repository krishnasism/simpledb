mod cache;
mod database;
mod file_manager;
use database::Collection;
use pyo3::prelude::*;

#[pyfunction]
fn get_collection(x: String) -> Collection {
    let mut database = Collection::new(x);
    database.create();
    database
}

#[pymodule]
#[pyo3(name = "simpledb")]
fn simpledb(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_collection, m)?)?;
    Ok(())
}
