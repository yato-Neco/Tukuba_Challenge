use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};


/*
python 呼び出せるコード。
説明はまた今度
*/



#[test]
pub fn python() -> PyResult<()> {
    Python::with_gil(|py| {
        let syspath: &PyList =
            pyo3::PyTryInto::try_into(py.import("sys").unwrap().getattr("path").unwrap()).unwrap();
        syspath.insert(0, "py_scripts").unwrap();

        let script = py.import(format!("{}", "test").as_str())?.getattr("CallTFlite")?;


        let tflite = script.call1(()).unwrap();

        tflite.call_method0("load_mode_label")?;
        
        

        let mp =  tflite.call_method0("start")?;

        let a:Vec<[f64;6]>  = mp.extract()?;

        //[Object(id=17, score=0.8515625, bbox=BBox(xmin=392, ymin=14, xmax=873, ymax=670))]
        //(&str,usize),(&str,f64), (&str,(&str, usize, &str, usize, &str, usize, &str, usize))

        for i in a.iter() {
            println!("{:?}",i);
        }



        
        Ok(())
    })
}