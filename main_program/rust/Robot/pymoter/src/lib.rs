use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};


#[test]
pub fn python() -> PyResult<()> {
    Python::with_gil(|py| {
        let syspath: &PyList =
            pyo3::PyTryInto::try_into(py.import("sys").unwrap().getattr("path").unwrap()).unwrap();
        syspath.insert(0, "py_scripts").unwrap();

        let script = py.import(format!("{}", "moter").as_str())?.getattr("Moter")?;


        let tflite = script.call1(()).unwrap();

        tflite.call_method0("left")?;
        
        

        //let mp =  tflite.call_method0("start")?;

  

        
        Ok(())
    })
}