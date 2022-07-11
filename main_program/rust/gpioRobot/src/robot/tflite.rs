use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};


/*
python 呼び出せるコード。
説明はまた今度
*/
pub fn python() -> PyResult<()> {
    Python::with_gil(|py| {
        let syspath: &PyList =
            pyo3::PyTryInto::try_into(py.import("sys").unwrap().getattr("path").unwrap()).unwrap();
        syspath.insert(0, "scripts").unwrap();

        let script = py.import(format!("{}", "test").as_str())?.getattr("CallTFlite")?;


        let tflite = script.call1(()).unwrap();

       
    
        let mp =  tflite.call_method0("start")?;

        println!("{:?}",mp);
        Ok(())
    })
}