
use pyo3::{prelude::*, wrap_pymodule};

mod docstring;
//::{parse, Docstring, _get_sections};

#[pyfunction]
fn hello_from_bin() -> String {
    "Hello from ruff-docstrings-complete!".to_string()
}

#[pyfunction]
fn my_hello() {
    println!("Hello from Ali!");
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(py: Python<'_>,m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bin, m)?)?;
    m.add_function(wrap_pyfunction!(my_hello, m)?)?;
    let submodule = PyModule::new_bound(py, "docstring")?;
    submodule.add_class::<docstring::Docstring>()?;
    submodule.add_class::<docstring::_Section>()?;

    submodule.add_function(wrap_pyfunction!(docstring::_get_sections, submodule.clone())?)?;
    submodule.add_function(wrap_pyfunction!(docstring::parse, submodule.clone())?)?;

    m.add_submodule(&submodule)?;
    Ok(())
}