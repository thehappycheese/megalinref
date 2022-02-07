use pyo3::{ToPyObject, PyObject, Python, FromPyObject, PyResult, PyAny};
use serde::{Serialize, Deserialize};


/// The carriageway of a road.
/// 
/// Although rust does not treat Enums as a bitflag, we will assume that python can treat them as such.
/// 
/// ```
/// L = 0b0100
/// S = 0b0010
/// R = 0b0001
/// ```
/// 
/// > Note: The cwy field is ordered `L` then `S` then `R`
/// 
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
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

// impl ToPyObject for Cwy{
//     fn to_object(&self, py: Python) -> PyObject {
//         (*self as u32).to_object(py)
//     }
// }

impl ToPyObject for Cwy{
    fn to_object(&self, py: Python) -> PyObject {
        match self{
            Cwy::L => "Left".to_object(py),
            Cwy::S => "Single".to_object(py),
            Cwy::R => "Right".to_object(py),
        }
    }
}
