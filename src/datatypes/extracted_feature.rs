use pyo3::prelude::*;
use pyo3::types::{PyDict};
use serde::{Serialize, Deserialize};
use super::{ExtractedLineString, ExtractedProperties};


#[derive(Serialize, Deserialize)]
pub struct ExtractedFeature {
    pub index:usize,
    pub properties:ExtractedProperties,
    pub geometry:ExtractedLineString
}


impl ExtractedFeature{
    // extract from pyobject with index set
    pub fn from_pyobject_with_index(obj: &PyAny, index:usize) -> PyResult<Self> {
        let dict = obj.extract::<&PyDict>()?;
        let properties = dict.get_item("properties").unwrap().extract::<ExtractedProperties>()?;
        let geometry = dict.get_item("geometry").unwrap().extract::<ExtractedLineString>()?;
        Ok(Self{
            index,
            properties,
            geometry
        })
    }
}


impl<'a> FromPyObject<'a> for ExtractedFeature{
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        let dict = obj.extract::<&PyDict>()?;
        let properties = dict.get_item("properties").unwrap().extract::<ExtractedProperties>()?;
        let geometry = dict.get_item("geometry").unwrap().extract::<ExtractedLineString>()?;
        Ok(Self{
            index:0,
            properties,
            geometry
        })
    }
}
