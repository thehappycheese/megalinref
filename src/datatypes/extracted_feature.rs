use pyo3::prelude::*;
use pyo3::types::{PyDict};
//use serde::{Serialize, Deserialize};
use super::{
    //ExtractedLineString,
    ExtractedProperties
};


//#[derive(Serialize, Deserialize)]
pub struct ExtractedFeature {
    //pub properties:ExtractedProperties,
    //pub geometry:ExtractedLineString
}

impl PartialEq for ExtractedFeature {
    fn eq(&self, other: &Self) -> bool {
        //self.properties == other.properties
        return false;
    }
}

// PartialOrd is needed to satisfy the Ord trait.
impl PartialOrd for ExtractedFeature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(std::cmp::Ordering::Equal);
        //Some(self.properties.cmp(&other.properties))
    }
}

// Eq is needed to satisfy the Ord trait.
impl Eq for ExtractedFeature{}

// Allows easy sorting
impl Ord for ExtractedFeature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return std::cmp::Ordering::Equal;
        //self.properties.cmp(&other.properties)
    }
}


impl<'a> FromPyObject<'a> for ExtractedFeature{
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        let dict = match obj.extract::<&PyDict>(){
            Ok(dict) => dict,
            Err(_) => return Err(pyo3::exceptions::PyException::new_err(
                "All items in 'features' must be of type dict"
            )),
        };

        let properties = match dict.get_item("properties"){
            Some(properties) => properties,
            None => return Err(pyo3::exceptions::PyException::new_err(
                "Unable to find the 'properties' item on one of the features."
            )),
        };

        let properties = match properties.extract::<ExtractedProperties>() {
            Ok(properties) => properties,
            Err(x) => return Err(x),
        };

        //let geometry = dict.get_item("geometry").unwrap().extract::<ExtractedLineString>()?;
        Ok(Self{
            // properties,
            // geometry
        })
    }
}
