use pyo3::prelude::*;
use pyo3::types::{PyDict};
use serde::{Serialize, Deserialize};
use super::{
    ExtractedLineString,
    ExtractedProperties
};


#[derive(Serialize, Deserialize)]
pub struct ExtractedFeature {
    pub properties:ExtractedProperties,
    pub geometry:ExtractedLineString
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

        let geometry = match dict.get_item("geometry"){
            Some(geometry) => match geometry.extract::<ExtractedLineString>() {
                Ok(geometry) => geometry,
                Err(x) => return Err(x),
            },
            None => return Err(pyo3::exceptions::PyException::new_err(
                "Unable to find the 'geometry' item on one of the features."
            )),
        };
            
        
        Ok(Self{
            properties,
            geometry
        })
    }
}
