use geo::{LineString};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use rstar::{RTreeObject, AABB, PointDistance};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MyLineString(pub LineString<f64>, pub usize);

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
        let result = MyLineString(LineString::from(coords), 0usize);
        Ok(result)
    }
}


impl RTreeObject for MyLineString {
    type Envelope = AABB<geo_types::Point<f64>>;
    fn envelope(&self) -> Self::Envelope {
        self.0.envelope().into()
    }
}

impl PointDistance for MyLineString {
    fn distance_2(&self, point: &geo::Point<f64>) -> f64 {
        self.0.distance_2(point)
    }
}