use bincode;

use geo::line_interpolate_point::LineInterpolatePoint;
use geo::line_locate_point::LineLocatePoint;
use geo::{point};
use geo::prelude::EuclideanDistance;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyTuple, PyList};

use rayon::prelude::*;


use serde::{Serialize, Deserialize};

use crate::datatypes::{ExtractedFeature, RoadSectionsByCarriageway};
use crate::util::convert_degrees_to_metres;

use std::collections::HashMap;
use std::collections::hash_map::Entry;



#[pyclass]
#[derive(Serialize, Deserialize)]
pub struct Lookup {
    features: Vec<ExtractedFeature>,
    index: HashMap<String, RoadSectionsByCarriageway>,
}

#[pymethods]
impl Lookup {
    /// (lat, lon, cwy, network_type, /)
    /// --
    ///
    /// A basic one-at-a-time lookup of the closest feature to the given lat/lon
    /// May be slow for bulk lookups
    #[pyo3(text_signature = "(lat, lon, cwy, network_type, /)")]
    pub fn road_slk_from_coordinate(
        &self,
        lat: f64,
        lon: f64,
        carriageways: u8,
        network_type: u8,
        py: Python,
    ) -> PyResult<PyObject> {
        let pnt = point!(x: lon, y: lat);

        let (index, distance) = self
            .features
            .par_iter()
            .enumerate()
            .filter(|(_index, ExtractedFeature { properties, .. })| {
                ((properties.cwy as u8) & carriageways) != 0
                    && ((properties.network_type as u8) & network_type) != 0
            })
            .map(|(index, feature)| Some((index, feature.geometry.0.euclidean_distance(&pnt))))
            .reduce(
                || None,
                |a, b| match (a, b) {
                    (None, b) => b,
                    (a, None) => a,
                    (Some((a_index, a_dist)), Some((b_index, b_dist))) => {
                        if a_dist < b_dist {
                            Some((a_index, a_dist))
                        } else {
                            Some((b_index, b_dist))
                        }
                    }
                },
            )
            .unwrap();

        let feature = &self.features[index];

        let distance_along_object = feature.geometry.0.line_locate_point(&pnt).unwrap();

        let feature_dict = PyDict::new(py);
        feature_dict.set_item("feature", feature.properties.to_object(py))?;
        feature_dict.set_item(
            "slk",
            (feature.properties.slk_from
                + (feature.properties.slk_to - feature.properties.slk_from)
                    * distance_along_object)
                .to_object(py),
        )?;
        feature_dict.set_item(
            "true",
            (feature.properties.true_from
                + (feature.properties.true_to - feature.properties.true_from)
                    * distance_along_object)
                .to_object(py),
        )?;
        feature_dict.set_item("distance_metres", convert_degrees_to_metres(distance))?;
        Ok(feature_dict.to_object(py))
    }

    pub fn coordinate_from_road_slk(
        &self,
        road: &str,
        slk: f64,
        carriageways: u8,
        py: Python,
    ) -> PyResult<PyObject> {
        let list_of_lists:Vec<PyObject> = self
            .index
            .get(road)
            .unwrap()
            .iter_matching_carriageways(carriageways)
            .filter_map(|(_cwy, index_range)| {
                let list_of_points:Vec<PyObject> = self
                .features[index_range]
                .into_iter()
                .filter_map(|feature|{
                    if feature.properties.slk_from <= slk && slk <= feature.properties.slk_to {
                        let fraction = (slk - feature.properties.slk_from) / (feature.properties.slk_to - feature.properties.slk_from);
                        match feature.geometry.0.line_interpolate_point(fraction){
                            Some(coordinate) => Some(PyTuple::new(py, &[coordinate.x().to_object(py), coordinate.y().to_object(py)]).to_object(py)),
                            None=>None
                        }
                    }else{
                        None
                    }
                })
                .collect();
                Some(PyList::new(py, list_of_points).to_object(py))
            }).collect();
        Ok(PyList::new(py, list_of_lists).into())
        
    }

    pub fn to_binary(&self, py: Python) -> PyResult<PyObject> {
        let encoded = bincode::serialize(&self).unwrap();
        let result;
        result = PyBytes::new_with(py, encoded.len(), |buffer| {
            buffer.copy_from_slice(&encoded);
            Ok(())
        });
        let result = result.unwrap();
        Ok(result.to_object(py))
    }

    #[staticmethod]
    pub fn from_binary(input: &PyBytes) -> PyResult<Self> {
        let lookup:Self = bincode::deserialize(input.as_bytes()).unwrap();
        Ok(lookup)
    }

    #[staticmethod]
    pub fn from_dict(python_dictionary: &PyDict) -> PyResult<Self> {
        let mut features: Vec<ExtractedFeature> = python_dictionary
            .get_item("features")
            .unwrap()
            .extract::<Vec<&PyAny>>()?
            .into_iter()
            .map(|feature| feature.extract::<ExtractedFeature>().unwrap())
            .collect();
        
        // Notes:
        // - .sort() is stable; order of equal elements is preserved.
        // - The order of input elements is currently unknown.
        // - the sorted Ord trait of ExtractedFeature currently does not look at slk or true.
        features.sort(); 

        let index = Self::build_index(&features);

        Ok(Self { features, index })
    }


    pub fn get_index(&self, py:Python)-> PyResult<PyObject>{
        let index_dict = PyDict::new(py);
        for (key, value) in &self.index {
            index_dict.set_item(key, value.to_object(py))?;
        }
        Ok(index_dict.to_object(py))
    }
        
}

impl Lookup {
    fn build_index(features: & Vec<ExtractedFeature>) -> HashMap<String, RoadSectionsByCarriageway> {
        let mut result:HashMap<String, RoadSectionsByCarriageway> = HashMap::new();

        // result.insert("".into(), RoadSectionsByCarriageway::new(Some((1,2)), None, None));
        // return result;

        let mut current_slice_start = 0;

        for i in 1..features.len() {
            let a_feature = &features[i-1];
            let b_feature = &features[i];
            let b_feature_is_new_road = a_feature.properties.road != b_feature.properties.road;
            let b_feature_is_new_cwy  = a_feature.properties.cwy  != b_feature.properties.cwy;
            
            if b_feature_is_new_road || b_feature_is_new_cwy {
                // the into() function on the next line is doing magic that I don't quite understand. Maybe its better than my previous solution which was .clone() ?
                match result.entry(b_feature.properties.road.clone()) {
                    Entry::Vacant(e) => {
                        e.insert(RoadSectionsByCarriageway::new_from_cwy(
                            &a_feature.properties.cwy,
                            (current_slice_start, i),
                        ));
                    }
                    Entry::Occupied(mut e) => {
                        e.insert(
                            e
                            .get()
                            .with_updated_cwy(
                                &a_feature.properties.cwy,
                                 (current_slice_start, i)
                            ),
                        );
                    }
                }
                current_slice_start = i;
            }
        }
        result
    }
}
