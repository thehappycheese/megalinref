use pyo3::{ToPyObject, PyObject, Python, FromPyObject, PyResult, PyAny};
use serde::{Serialize, Deserialize};


#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Cwy {
    L = 0b0000_0100,
    S = 0b0000_0010,
    R = 0b0000_0001,
}

impl<'a> FromPyObject<'a> for Cwy{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        ob.extract::<&str>().map(|x| match x {
            "Left" => Cwy::L,
            "Single" => Cwy::S,
            "Right" => Cwy::R,
            _ => panic!("Invalid value for Cwy")
        })
    }
}

impl ToPyObject for Cwy{
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u32).to_object(py)
    }
}
