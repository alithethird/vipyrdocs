
use pyo3::{prelude::*};

mod docstring;
//::{parse, Docstring, _get_sections};

mod plugin;
pub mod constants;
pub mod rule_engine;
mod test_rule_engine;

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
    m.add_function(wrap_pyfunction!(rule_engine::apply_rules, m)?)?;

    let submodule = PyModule::new_bound(py, "docstring")?;
    submodule.add_class::<docstring::Docstring>()?;
    submodule.add_class::<docstring::_Section>()?;

    submodule.add_function(wrap_pyfunction!(docstring::_get_sections, submodule.clone())?)?;
    submodule.add_function(wrap_pyfunction!(docstring::parse, submodule.clone())?)?;

    m.add_submodule(&submodule)?;
    let constants = PyModule::new_bound(py, "constants")?;
    let _ = constants.add("ERROR_CODE_PREFIX", constants::ERROR_CODE_PREFIX);
    let _ = constants.add("MORE_INFO_BASE", constants::MORE_INFO_BASE);
    let _ = constants.add("DOCSTR_MISSING_CODE", constants::docstr_missing_code());
    let _ = constants.add("DOCSTR_MISSING_MSG", constants::docstr_missing_msg());
    m.add_submodule(&constants)?;

    Ok(())
}