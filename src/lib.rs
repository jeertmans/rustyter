use pyo3::prelude::*;

/// Formats the sum of two integers as string.
#[pyfunction]
#[pyo3(text_signature = "(a, b, /)")]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Reads iterator and returns length.
#[pyfunction]
#[pyo3(text_signature = "(it, /)")]
fn length(it: Vec<PyObject>) -> PyResult<usize> {
    Ok(it.iter().count())
}
//fn it(obj: &T) -> PyIterator<'p> {
//
//}

/// A Python module implemented in Rust.
#[pymodule]
fn rustyter(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(length, m)?)?;

    Ok(())
}
