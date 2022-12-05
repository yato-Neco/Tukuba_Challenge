/*



use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

struct PyMoter {
}

impl PyMoter {
    

    pub fn _front(pyfun: &PyAny,r_duty:f64,l_duty:f64) {
        pyfun.call_method1("front",(r_duty,l_duty)).unwrap();
    }

    pub fn _back(pyfun: &PyAny,r_duty:f64,l_duty:f64) {
        pyfun.call_method1("back",(r_duty,l_duty)).unwrap();

    }

    pub fn _left(pyfun: &PyAny,r_duty:f64,l_duty:f64) {
        pyfun.call_method1("left",(r_duty,l_duty)).unwrap();

    }

    pub fn _right(pyfun: &PyAny,r_duty:f64,l_duty:f64) {
        pyfun.call_method1("right",(r_duty,l_duty)).unwrap();

    }

    pub fn _stop(pyfun: &PyAny) {
        pyfun.call_method0("stop").unwrap();

    }
}

#[test]
pub fn python() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let syspath: &PyList =
        pyo3::PyTryInto::try_into(py.import("sys").unwrap().getattr("path").unwrap()).unwrap();
    syspath.insert(0, "py_scripts").unwrap();

    let script = py
        .import(format!("{}", "moter").as_str())
        .unwrap()
        .getattr("Moter")
        .unwrap();
    
    let pyfun = script.call1(()).unwrap();

    
    PyMoter::_front(pyfun,0.0,0.0);
    
}
*/