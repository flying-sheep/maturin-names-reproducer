#[cfg_attr(feature = "pyo3", pyo3::pyfunction)]
pub fn attr_names() -> Vec<&'static str> {
    vec!["_draw_", "_ldraw_", "_hdraw_", "_tdraw_", "_hldraw_", "_tldraw_"]
}

#[cfg(feature = "pyo3")]
#[pyo3::pymodule]
#[pyo3(name = "xdot_rs")]
pub fn pymodule(_py: pyo3::Python, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
    m.add_function(pyo3::wrap_pyfunction!(attr_names, m)?)?;
    Ok(())
}
