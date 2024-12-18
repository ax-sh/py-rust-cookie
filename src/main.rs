use pyo3::prelude::PyModule;
use pyo3::{PyResult, Python};

fn main() -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        // Import the Python 'math' module
        let math = PyModule::import(py, "math")?;
        println!("{}",math);
        //
        // // Call 'math.sqrt' with an argument
        // let result: f64 = math.get("sqrt")?.call1((16,))?.extract()?;
        //
        // println!("The square root of 16 is: {}", result);

        Ok(())
    })
}