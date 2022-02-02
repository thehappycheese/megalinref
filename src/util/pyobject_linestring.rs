use geo::LineString;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use rstar::{AABB, RTreeObject};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyLineString(LineString<f64>);

impl<'a> FromPyObject<'a> for MyLineString {
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
        let result = MyLineString(LineString::from(coords));
        Ok(result)
    }
}

impl RTreeObject for MyLineString {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.envelope().into()
    }
}
