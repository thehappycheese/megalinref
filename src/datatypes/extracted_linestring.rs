use pyo3::types::PyDict;

use geo::{LineString};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedLineString(pub LineString<f64>);

impl<'a> FromPyObject<'a> for ExtractedLineString {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {

        let linestring_dict = obj.extract::<&PyDict>()?;
        let coords = linestring_dict
            .get_item("coordinates")
            .unwrap()
            .extract::<Vec<&PyAny>>()?;

        let coords: Vec<(f64, f64)> = coords
            .into_iter()
            .map(|item| {
                let list = item.extract::<Vec<f64>>().unwrap();
                (list[0], list[1])
            })
            .collect();
        let result = ExtractedLineString(LineString::from(coords));
        Ok(result)
    }
}
