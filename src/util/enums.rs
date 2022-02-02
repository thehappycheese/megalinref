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

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum NetworkType {
    State_Road                 = 0b0000_0001,
    Local_Road                 = 0b0000_0010,
    Miscellaneous_Road         = 0b0000_0100,
    Main_Roads_Controlled_Path = 0b0000_1000,
    Proposed_Road              = 0b0001_0000,
    Crossover                  = 0b0010_0000,
}

impl<'a> FromPyObject<'a> for NetworkType{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        ob.extract::<&str>().map(|x| match x {
            "State Road" => NetworkType::State_Road,
            "Local Road" => NetworkType::Local_Road,
            "Miscellaneous Road" => NetworkType::Miscellaneous_Road,
            "Main Roads Controlled Path" => NetworkType::Main_Roads_Controlled_Path,
            "Proposed Road" => NetworkType::Proposed_Road,
            "Crossover" => NetworkType::Crossover,
            _ => panic!("Invalid value for NetworkType")
        })
    }
}


impl ToPyObject for NetworkType{
    fn to_object(&self, py: Python) -> PyObject {
        (*self as u32).to_object(py)
    }
}
