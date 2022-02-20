use pyo3::{
    ToPyObject, PyObject, Python, 
    FromPyObject, PyResult, PyAny, PyErr, exceptions
};

use serde::{Serialize, Deserialize};


// define enum members up here so we don't mix them up when we have to type out the numbers multiple times.
const CWY_LEFT:u8   = 0b0000_0100;
const CWY_SINGLE:u8 = 0b0000_0010;
const CWY_RIGHT:u8  = 0b0000_0001;

/// The carriageway of a road.
/// 
/// Rust does not treat Enums as a bitflags, but we can abuse casting to and from u8 to make it do so anyway.
/// simply making the CWY_LEFT, CWY_SINGLE, and CWY_RIGHT constants public we could achieve what rust needs.
/// But so that we can impl FromPyObject for Cwy, we will just make it an enum.
/// 
/// Python has no enums, except for the abomination baseclass in the standard library
/// So we will use a dict in python to store ints and call it done.
/// 
/// ```
/// assert!((Cwy::Left   as u8) == 0b000_0100);
/// assert!((Cwy::Single as u8) == 0b000_0010);
/// assert!((Cwy::Right  as u8) == 0b000_0001);
/// ```
/// 
/// > Note: The cwy field is ordered `L` then `S` then `R`
/// 
#[derive(
    Serialize, Deserialize,
    Copy, Clone,
    PartialEq, Eq,
    PartialOrd, Ord
)]
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
            _        => Err(exceptions::PyValueError::new_err::<String>("Invalid value for CWY".into()))
        }
    }
}


impl<'a> FromPyObject<'a> for Cwy{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        match ob.extract::<&str>(){
            Ok(s) => Cwy::try_from(s),
            Err(_) => return Err(pyo3::exceptions::PyValueError::new_err("'CWY' must be of type str"))
        }
    }
}


// impl TryFrom<u8> for Cwy{
//     type Error = PyErr;
//     fn try_from(i: u8) -> PyResult<Self> {
//         match i {
//             CWY_LEFT   => Ok(Cwy::Left),
//             CWY_SINGLE => Ok(Cwy::Single),
//             CWY_RIGHT  => Ok(Cwy::Right),
//             _     => Err(exceptions::PyValueError::new_err::<String>("Invalid value for CWY".into()))
//         }
//     }
// }


impl ToPyObject for Cwy{
    fn to_object(&self, py: Python) -> PyObject {
        match self{
            Cwy::Left => "Left".to_object(py),
            Cwy::Single => "Single".to_object(py),
            Cwy::Right => "Right".to_object(py),
        }
    }
}
