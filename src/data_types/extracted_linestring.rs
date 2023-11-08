use pyo3::types::PyDict;

use geo::LineString;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize, Clone)]
pub struct ExtractedLineString(
    pub LineString<f64>
);


impl<'a> FromPyObject<'a> for ExtractedLineString {
    fn extract(obj: &'a PyAny) -> PyResult<Self> {

        let linestring_dict = match obj.extract::<&PyDict>(){
            Ok(linestring_dict) => linestring_dict,
            Err(_) => return Err(pyo3::exceptions::PyException::new_err(
                "All 'geometry' in each 'features' must be of type dict"
            )),
        };

        let coords = match linestring_dict.get_item("coordinates"){
            Ok(Some(coords)) => match coords.extract::<Vec<&PyAny>>(){
                Ok(coords) => coords,
                Err(_) => return Err(pyo3::exceptions::PyException::new_err(
                    "Unable to interpret 'coordinates' in one of the 'features'"
                )),
            },
            _ => return Err(pyo3::exceptions::PyException::new_err(
                "Unable to find the 'coordinates' item on one of the features."
            )),
        };

        let coords = coords.iter().map(|coord| {
            match coord.extract::<Vec<f64>>(){
                Ok(coord) => Ok((coord[0], coord[1])),
                Err(_) => return Err(pyo3::exceptions::PyException::new_err(
                    "Unable to interpret one of the 'coordinates' in one of the 'features'"
                )),
            }
        }).collect::<PyResult<Vec<(f64, f64)>>>()?;

        let result = ExtractedLineString(LineString::from(coords));

        Ok(result)
    }
}



