#![allow(clippy::deprecated)]
use pyo3::prelude::*;
use tokenizations::{get_alignments, get_charmap, Alignment, CharMap};

#[pymodule]
#[pyo3(name = "tokenizations")]
fn tokenizations_(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", "0.9.1")?;

    #[pyfn(m)]
    #[pyo3(name = "get_alignments")]
    pub fn get_alignments_py(
        _py: Python,
        a: Vec<&str>,
        b: Vec<&str>,
    ) -> PyResult<(Alignment, Alignment)> {
        Ok(get_alignments(&a, &b))
    }

    #[pyfn(m)]
    #[pyo3(name = "get_charmap")]
    pub fn get_charmap_py(_py: Python, a: &str, b: &str) -> PyResult<(CharMap, CharMap)> {
        Ok(get_charmap(a, b))
    }

    Ok(())
}
