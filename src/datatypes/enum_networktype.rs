
use pyo3::{
    ToPyObject,
    PyObject,
    Python,
    FromPyObject,
    PyResult,
    PyAny,
};
use serde::{Serialize, Deserialize};

#[derive(
    Serialize, Deserialize,
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

impl NetworkType {
    pub fn matches_filter(&self, filter:u8) -> bool {
        return ((*self as u8) & filter) != 0
    }
}


impl<'a> FromPyObject<'a> for NetworkType{
    fn extract(input: &'a PyAny) -> PyResult<Self> {
        match input.extract::<&str>(){
            Ok(thestr)=>match thestr.try_into() {   
                Ok(variant) => Ok(variant),
                Err(msg)=> Err(pyo3::exceptions::PyException::new_err(
                    format!("Unable to extract NETWORK_TYPE as string: {msg}")
                ))
            },
            Err(_) => Err(pyo3::exceptions::PyException::new_err(
                "Unable to extract NETWORK_TYPE as string",
            )),
        }
    }
}

impl TryFrom<&str> for NetworkType {
    type Error = String;
    fn try_from(input_string:&str)->Result<Self, Self::Error>{
        match input_string.to_lowercase().as_ref() {
            "state road"                 => Ok(NetworkType::State_Road),
            "local road"                 => Ok(NetworkType::Local_Road),
            "miscellaneous road"         => Ok(NetworkType::Miscellaneous_Road),
            "main roads controlled path" => Ok(NetworkType::Main_Roads_Controlled_Path),
            "proposed road"              => Ok(NetworkType::Proposed_Road),
            "crossover"                  => Ok(NetworkType::Crossover),
            other=>{
                let error_text_to_display:String = other.chars().into_iter().take(50).collect();
                Err(format!("Could not parse NetworkType. Expected state road, local road, miscellaneous road, main roads controlled path, proposed road, crossover (Not case Sensitive)  but found '{error_text_to_display}'"))
            }
        }
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