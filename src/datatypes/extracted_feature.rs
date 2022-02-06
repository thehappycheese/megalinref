use pyo3::prelude::*;
use pyo3::types::{PyDict};
use serde::{Serialize, Deserialize};
use super::{ExtractedLineString, ExtractedProperties};


#[derive(Serialize, Deserialize)]
pub struct ExtractedFeature {
    pub properties:ExtractedProperties,
    pub geometry:ExtractedLineString
}

impl PartialEq for ExtractedFeature {
    fn eq(&self, other: &Self) -> bool {
        self.properties == other.properties
    }
}

// PartialOrd is needed to satisfy the Ord trait.
impl PartialOrd for ExtractedFeature {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.properties.cmp(&other.properties))
    }
}

// Eq is needed to satisfy the Ord trait.
impl Eq for ExtractedFeature{}

// Allows easy sorting
impl Ord for ExtractedFeature {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.properties.cmp(&other.properties)
    }
}


impl<'a> FromPyObject<'a> for ExtractedFeature{
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        let dict = obj.extract::<&PyDict>()?;
        let properties = dict.get_item("properties").unwrap().extract::<ExtractedProperties>()?;
        let geometry = dict.get_item("geometry").unwrap().extract::<ExtractedLineString>()?;
        Ok(Self{
            properties,
            geometry
        })
    }
}
