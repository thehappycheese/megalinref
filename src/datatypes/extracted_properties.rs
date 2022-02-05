
use pyo3::types::PyDict;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Cwy, NetworkType};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedProperties{
    pub road:String,
    pub cwy:Cwy,
    pub slk_from:f64,
    pub slk_to:f64,
    pub true_from:f64,
    pub true_to:f64,
    pub network_type:NetworkType,
}

impl<'a> FromPyObject<'a> for ExtractedProperties{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let ob = ob.cast_as::<PyDict>()?;
        Ok(Self{
            road:         ob.get_item("ROAD")            .unwrap().extract::<String>()?,
            cwy:          ob.get_item("CWY")             .unwrap().extract::<Cwy>()?,
            slk_from:     ob.get_item("START_SLK")       .unwrap().extract::<f64>()?,
            slk_to:       ob.get_item("END_SLK")         .unwrap().extract::<f64>()?,
            true_from:    ob.get_item("START_TRUE_DIST") .unwrap().extract::<f64>()?,
            true_to:      ob.get_item("END_TRUE_DIST")   .unwrap().extract::<f64>()?,
            network_type: ob.get_item("NETWORK_TYPE")    .unwrap().extract::<NetworkType>()?,
        })
    }
}