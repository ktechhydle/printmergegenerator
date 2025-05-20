use pyo3::prelude::*;
mod framework;

// test function to see if bindings are correct
#[pyfunction]
fn test() -> PyResult<()> {
    println!("Hello from Rust!");
    Ok(())
}

// wrap the functions to module
#[pymodule]
fn print_merge_generator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_function(wrap_pyfunction!(framework::generator::generate_numbers, m)?)?;
    Ok(())
}
