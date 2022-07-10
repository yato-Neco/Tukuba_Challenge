use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyList};

fn main() -> PyResult<()> {
    Python::with_gil(|py| {
        // scripts ディレクトリをパスに追加する
        let syspath: &PyList =
            pyo3::PyTryInto::try_into(py.import("sys").unwrap().getattr("path").unwrap()).unwrap();
        syspath.insert(0, "scripts").unwrap();

        // スクリプトファイルの echo 関数を読み込む
        let script = py.import(format!("{}", "test").as_str())?.getattr("ExampleClass")?;


        let example = script.call1(()).unwrap();
        // 関数を呼び出す

       
    
        let mp =  example.call_method0("hello")?;

        println!("{}",mp);
        Ok(())
    })
}