use pyo3::{
    ToPyObject, PyObject, Python, 
    FromPyObject, PyResult, PyAny, PyErr, exceptions
};

use serde::{Serialize, Deserialize};

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
/// 
#[repr(u8)]
#[derive(
    Serialize, Deserialize,
    Copy, Clone,
    PartialEq, Eq,
    PartialOrd, Ord
)]
pub enum Cwy {
    Left   = 0b0000_0100,
    Single = 0b0000_0010,
    Right  = 0b0000_0001,
}

impl Cwy{

    pub fn all()->u8 {
        Cwy::Left as u8 | 
        Cwy::Single as u8 |
        Cwy::Right as u8
    }

    pub fn matches_filter(&self, filter:u8) -> bool{
        return ((*self as u8) & filter) != 0
    }

    /// Convert a u8 representation of a Carriageway filter
    /// into a string which can be used for error messages
    pub fn filter_to_string(filter:u8) -> String {
        #[allow(non_snake_case)]
        let L = match Cwy::Left.matches_filter(filter) {
            true  => "L",
            false => ""
        };
        #[allow(non_snake_case)]
        let R = match Cwy::Right.matches_filter(filter) {
            true  => "R",
            false => ""
        };
        #[allow(non_snake_case)]
        let S = match Cwy::Single.matches_filter(filter) {
            true  => "S",
            false => ""
        };
        let result = format!("{L}{R}{S}");
        match &result[..] {
            "" => "NoCway".into(),
            _  => result
        }
    }
}


impl TryFrom<&str> for Cwy{
    type Error = PyErr;
    fn try_from(s: &str) -> PyResult<Self> {
        match s.to_lowercase().as_ref() {
            "left"   => Ok(Cwy::Left),
            "single" => Ok(Cwy::Single),
            "right"  => Ok(Cwy::Right),
            "l"      => Ok(Cwy::Left),
            "s"      => Ok(Cwy::Single),
            "r"      => Ok(Cwy::Right),
            other    => Err(exceptions::PyValueError::new_err::<String>(format!("Invalid value for CWY: Expected left, single, right, l, r, s but found '{other}' (Not case sensitive)")))
        }
    }
}

impl std::fmt::Display for Cwy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left   => write!(f, "Left"),
            Self::Single => write!(f, "Single"),
            Self::Right  => write!(f, "Right"),
        }?;
        Ok(())
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


// Not sure why I disabled this; I think it is wrong; it should be for Vec<Cwy> since a single int can encode more than one cwy
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
            Cwy::Left   => "Left".to_object(py),
            Cwy::Single => "Single".to_object(py),
            Cwy::Right  => "Right".to_object(py),
        }
    }
}
