// use bincode;


use std::borrow::Borrow;

use geo::{
   point,
   line_interpolate_point::LineInterpolatePoint,
   euclidean_distance::EuclideanDistance,
   line_locate_point::LineLocatePoint,
};

// use pyo3::exceptions::PyValueError;
use pyo3::{
    prelude::*,
    types::{
        // PyBytes,
        PyDict, PyList,
        // PyTuple,
        // PyList,
    }
};

use rayon::prelude::*;


// use serde::{Serialize, Deserialize};

use crate::datatypes::{
    ExtractedFeature,
//     RoadSectionsByCarriageway,
};
use crate::util::convert_degrees_to_metres;





#[pyclass]
pub struct Lookup {
    features: Vec<ExtractedFeature>,
}

#[pymethods]
impl Lookup {

    // #[new]
    // pub fn new(features:Py<Vec<i64>>) -> Self {
    //     Lookup {
    //         features,
    //     }
    // }

    #[staticmethod]
    pub fn from_dict(input:Py<PyDict>, py:Python) -> PyResult<Py<Self>> {
        //py.run("print('silly')",None, None)?;


        let arg_features = input
            .as_ref(py)
            .get_item("features");
        
        let arg_features = match arg_features {
            Some(features) => features,
            None => return Err(pyo3::exceptions::PyException::new_err("Unable to extract 'features' from input")),
        };

        let arg_features = match arg_features.cast_as::<PyList>() {
            Ok(features) => features,
            Err(_) => return Err(pyo3::exceptions::PyException::new_err("Unable to cast 'features' to list")),
        };

        if arg_features.len() == 0 {
            return Err(pyo3::exceptions::PyException::new_err("'features' list is empty"));
        }

        let result = Py::new(py, Self{
            features:Vec::with_capacity(arg_features.len()),
        })?;
        {
            let features:& mut Vec<ExtractedFeature> = & mut result.borrow_mut(py).features;

            for feature in arg_features.iter() {
                match feature.extract::<ExtractedFeature>(){
                    Ok(feature) => features.push(feature),
                    Err(x) => return Err(x),
                }
            }
        }
        Ok(result)
    }

    pub fn get_feature_count(&self, py:Python) -> PyResult<usize> {
        Ok(self.features.len())
    }

    pub fn road_slk_from_coordinate(
        &self,
        lat: f64,
        lon: f64,
        carriageways: u8,
        network_type: u8,
        py: Python,
    ) -> PyResult<PyObject> {
        return Err(pyo3::exceptions::PyException::new_err("Not Implemented"));
        // let pnt = point!(x: lon, y: lat);

        // let lookup_result = self
        //     .features
        //     .par_iter()
        //     .enumerate()
        //     .filter(|(_index, ExtractedFeature { properties, .. })| {
        //         ((properties.cwy as u8) & carriageways) != 0
        //             && ((properties.network_type as u8) & network_type) != 0
        //     })
        //     .map(|(index, feature)| Some((index, feature.geometry.0.euclidean_distance(&pnt))))
        //     .reduce(
        //         || None,
        //         |a, b| match (a, b) {
        //             (None, b) => b,
        //             (a, None) => a,
        //             (Some((a_index, a_dist)), Some((b_index, b_dist))) => {
        //                 if a_dist < b_dist {
        //                     Some((a_index, a_dist))
        //                 } else {
        //                     Some((b_index, b_dist))
        //                 }
        //             }
        //         },
        //     );
        
        // let (index, distance) = match lookup_result {
        //     Some(result) => result,
        //     None => return Err(pyo3::exceptions::PyException::new_err(concat!(
        //         "Failed to find any roads in dataset. ",
        //         "Thats weird because this function should find the nearest road even if the dataset contains one road. ",
        //         "This is likely because this Lookup object was constructed with an 'empty' dataset. ", 
        //         "Check the dictionary passed to .from_dict()"
        //     ))),
        // };
        

        // let feature = &self.features[index];

        // let distance_along_object = feature.geometry.0.line_locate_point(&pnt).unwrap();

        // let feature_dict = PyDict::new(py);
        // feature_dict.set_item("feature", feature.properties.to_object(py))?;
        // feature_dict.set_item(
        //     "slk",
        //     (feature.properties.slk_from
        //         + (feature.properties.slk_to - feature.properties.slk_from)
        //             * distance_along_object)
        //         .to_object(py),
        // )?;
        // feature_dict.set_item(
        //     "true",
        //     (feature.properties.true_from
        //         + (feature.properties.true_to - feature.properties.true_from)
        //             * distance_along_object)
        //         .to_object(py),
        // )?;
        // feature_dict.set_item("distance_metres", convert_degrees_to_metres(distance))?;
        // Ok(feature_dict.to_object(py))
    }

    

}
