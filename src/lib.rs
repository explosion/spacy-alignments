#![allow(clippy::deprecated)]
use pyo3::prelude::*;
use tokenizations::{get_alignments, get_charmap, Alignment, CharMap};

#[pyfunction]
#[pyo3(name = "get_alignments")]
pub fn get_alignments_py(
    a: Vec<String>,
    b: Vec<String>,
) -> PyResult<(Alignment, Alignment)> {
    let a_refs: Vec<&str> = a.iter().map(|s| s.as_str()).collect();
    let b_refs: Vec<&str> = b.iter().map(|s| s.as_str()).collect();
    Ok(get_alignments(&a_refs, &b_refs))
}

#[pyfunction]
#[pyo3(name = "get_charmap")]
pub fn get_charmap_py(a: String, b: String) -> PyResult<(CharMap, CharMap)> {
    Ok(get_charmap(&a, &b))
}

#[pymodule]
#[pyo3(name = "tokenizations")]
fn tokenizations_(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", "0.9.1")?;
    m.add_function(wrap_pyfunction!(get_alignments_py, m)?)?;
    m.add_function(wrap_pyfunction!(get_charmap_py, m)?)?;
    Ok(())
}
