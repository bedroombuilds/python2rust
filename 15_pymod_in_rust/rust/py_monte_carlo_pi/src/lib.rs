use cpython::{py_fn, py_module_initializer, PyResult, Python};
use monte_carlo_pi::monte_carlo_pi;

py_module_initializer!(montecarlopi, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(
        py,
        "sum_as_string",
        py_fn!(py, sum_as_string_py(a: i64, b: i64)),
    )?;
    m.add(py, "mcpi", py_fn!(py, mcpi_py(iterations: i64)))?;
    Ok(())
});

fn sum_as_string(a: i64, b: i64) -> String {
    format!("{}", a + b).to_string()
}

fn sum_as_string_py(_: Python, a: i64, b: i64) -> PyResult<String> {
    let out = sum_as_string(a, b);
    Ok(out)
}

fn mcpi_py(_: Python, iterations: i64) -> PyResult<(f64, String)> {
    let out = monte_carlo_pi(iterations as u32);
    Ok(out)
}
