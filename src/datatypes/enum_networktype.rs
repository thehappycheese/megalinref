
use pyo3::{ToPyObject, PyObject, Python, FromPyObject, PyResult, PyAny};
//use serde::{Serialize, Deserialize};

#[derive(
    //Serialize, Deserialize,
    Copy, Clone,  PartialEq, PartialOrd
)]
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
    fn extract(input: &'a PyAny) -> PyResult<Self> {
        let result = match input.extract::<&str>(){   
            "State Road"                 => Ok(NetworkType::State_Road),
            "Local Road"                 => Ok(NetworkType::Local_Road),
            "Miscellaneous Road"         => Ok(NetworkType::Miscellaneous_Road),
            "Main Roads Controlled Path" => Ok(NetworkType::Main_Roads_Controlled_Path),
            "Proposed Road"              => Ok(NetworkType::Proposed_Road),
            "Crossover"                  => Ok(NetworkType::Crossover),
            _                            => return Err(pyo3::exceptions::PyException::new_err(
                "Invalid value for NetworkType"
            ))
        }
        result
    }
}


// impl ToPyObject for NetworkType{
//     fn to_object(&self, py: Python) -> PyObject {
//         (*self as u32).to_object(py)
//     }
// }

impl ToPyObject for NetworkType{
    fn to_object(&self, py: Python) -> PyObject {
        match self{
            NetworkType::State_Road =>                 "State Road".to_object(py),
            NetworkType::Local_Road =>                 "Local Road".to_object(py),
            NetworkType::Miscellaneous_Road =>         "Miscellaneous Road".to_object(py),
            NetworkType::Main_Roads_Controlled_Path => "Main Roads Controlled Path".to_object(py),
            NetworkType::Proposed_Road =>              "Proposed Road".to_object(py),
            NetworkType::Crossover =>                  "Crossover".to_object(py),
        }
    }
}