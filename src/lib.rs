use pyo3::prelude::*;
use pyo3::types::PyModule;
use ssr_rs::Ssr;
use std::fs;

#[pyfunction]
fn render(js_path: &str, entry_point: &str, props: Option<&str>) -> PyResult<String> {
    let source = fs::read_to_string(js_path)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    Ssr::create_platform();
    let mut ssr = Ssr::from(source, entry_point).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
            "Error creating SSR platform: {}",
            e
        ))
    })?;
    let html = ssr.render_to_string(props).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Error rendering: {}", e))
    })?;
    Ok(html)
}

#[pymodule]
fn py_ssr(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render, m)?)?;
    Ok(())
}
