
use bincode;

use geo::line_locate_point::LineLocatePoint;
use geo::point;
use geo::prelude::EuclideanDistance;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyBytes};


use rayon::prelude::*;


use crate::datatypes::{
    ExtractedFeature
};
use crate::util::{
    convert_degrees_to_metres
};




#[pyclass]
pub struct SLKLookup{
    features:Vec<ExtractedFeature>
}

#[pymethods]
impl SLKLookup{
    #[new]
    pub fn new(python_dictionary:&PyDict) -> PyResult<Self>{
        let features:Vec<ExtractedFeature> = python_dictionary
        .get_item("features")
        .unwrap()
        .extract::<Vec<&PyAny>>()?
        .into_iter()
        .map(|feature| feature.extract::<ExtractedFeature>().unwrap())
        .collect();
        Ok(
            Self{features}
        )
    }

    
    /// A basic one-at-a-time lookup of the closest feature to the given lat/lon
    /// May be slow for bulk lookups
    #[pyo3(text_signature = "(lat, lon, cwy, network_type, /)")]
    pub fn lookup(&self, lat:f64, lon:f64, cwy:u8, network_type:u8, py:Python) -> PyResult<PyObject>{
        let pnt = point!(x:lon, y:lat);

        let (index, distance) = self
            .features
            .par_iter()
            .enumerate()
            .filter(|(_index, ExtractedFeature{properties, ..})| ((properties.cwy as u8) & cwy) != 0 && ((properties.network_type as u8) & network_type) != 0)
            .map(|(index, feature)| Some((index, feature.geometry.0.euclidean_distance(&pnt))))
            .reduce(|| None, |a, b|{
                match (a, b){
                    (None, b) => b,
                    (a, None) => a,
                    (Some((a_index, a_dist)), Some((b_index, b_dist))) => {
                        if a_dist < b_dist {
                            Some((a_index, a_dist))
                        }else{
                            Some((b_index, b_dist))
                        }
                    }
                }
            }).unwrap();
        
        let feature = &self.features[index];

        let distance_along_object = feature.geometry.0.line_locate_point(&pnt).unwrap();

        let feature_dict = PyDict::new(py);
        feature_dict.set_item("feature", feature.properties.to_object(py))?;
        feature_dict.set_item("slk",(feature.properties.slk_from + (feature.properties.slk_to - feature.properties.slk_from) * distance_along_object ).to_object(py))?;
        feature_dict.set_item("true",(feature.properties.true_from + (feature.properties.true_to - feature.properties.true_from) * distance_along_object).to_object(py))?;
        feature_dict.set_item("distance_metres", convert_degrees_to_metres(distance))?;
        Ok(feature_dict.to_object(py))
    }

    pub fn to_binary(&self, py:Python) -> PyResult<PyObject>{
        let encoded = bincode::serialize(&self.features).unwrap();
        let result;
        result = PyBytes::new_with(py, encoded.len(),|buffer|{
            buffer.copy_from_slice(&encoded);
            Ok(())
        });
        let result = result.unwrap();
        Ok(result.to_object(py))
    }

    #[staticmethod]
    pub fn from_binary(input:&PyBytes) -> PyResult<Self>{
        Ok(
            Self{
                features: bincode::deserialize(input.as_bytes()).unwrap()
            }
        )
    }
    
}