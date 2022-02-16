use pyo3::{ToPyObject, PyObject, Python, FromPyObject, PyResult, PyAny, PyErr, exceptions};

use serde::{Serialize, Deserialize};


// define enum members up here so we don't mix them up when we have to type out the numbers multiple times.
const CWY_LEFT:u8 = 0b0000_0100;
const CWY_SINGLE:u8 = 0b0000_0010;
const CWY_RIGHT:u8 = 0b0000_0001;

/// The carriageway of a road.
/// 
/// Although rust does not treat Enums as a bitflag, we will assume that python can treat them as such.
/// 
/// ```
/// Left   = 0b000_0100
/// Single = 0b000_0010
/// Right  = 0b000_0001
/// ```
/// 
/// > Note: The cwy field is ordered `L` then `S` then `R`
/// 
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum Cwy {
    Left   = CWY_LEFT   as isize,
    Single = CWY_SINGLE as isize,
    Right  = CWY_RIGHT  as isize,
}

impl TryFrom<&str> for Cwy{
    type Error = PyErr;
    fn try_from(s: &str) -> PyResult<Self> {
        match s {
            "Left"   => Ok(Cwy::Left),
            "Single" => Ok(Cwy::Single),
            "Right"  => Ok(Cwy::Right),
            "L"      => Ok(Cwy::Left),
            "S"      => Ok(Cwy::Single),
            "R"      => Ok(Cwy::Right),
            _        => Err(exceptions::PyValueError::new_err::<String>("Invalid value for Cwy".into()))
        }
    }
}

impl TryFrom<u8> for Cwy{
    type Error = PyErr;
    fn try_from(i: u8) -> PyResult<Self> {
        match i {
            CWY_LEFT => Ok(Cwy::Left),
            CWY_SINGLE => Ok(Cwy::Single),
            CWY_RIGHT => Ok(Cwy::Right),
            _     => Err(exceptions::PyValueError::new_err::<String>("Invalid value for Cwy".into()))
        }
    }
}

impl<'a> FromPyObject<'a> for Cwy{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        ob.extract::<Cwy>()
    }
}

impl ToPyObject for Cwy{
    fn to_object(&self, py: Python) -> PyObject {
        match self{
            Cwy::Left => "Left".to_object(py),
            Cwy::Single => "Single".to_object(py),
            Cwy::Right => "Right".to_object(py),
        }
    }
}
