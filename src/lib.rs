mod api;

use pyo3::prelude::*;

/// Asynchronous sleep function implemented in Rust.
#[pyfunction]
fn rust_sleep(py: Python, seconds: u64) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let duration = std::time::Duration::from_secs(seconds);
        tokio::time::sleep(duration).await;
        Ok(())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn ghsrch(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_sleep, m)?)?;
    Ok(())
}
