// use bincode;

use geo::line_interpolate_point::LineInterpolatePoint;
use geo::line_locate_point::LineLocatePoint;
use geo::{point};
use geo::prelude::EuclideanDistance;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{
    //PyBytes,
    PyDict,
    PyTuple,
    PyList,
};

use rayon::prelude::*;


// use serde::{Serialize, Deserialize};

use crate::datatypes::{
    ExtractedFeature,
    //RoadSectionsByCarriageway,
};
use crate::util::convert_degrees_to_metres;

//use std::collections::BTreeMap;
//use std::collections::btree_map::Entry;



#[pyclass]
// #[derive(Serialize, Deserialize)]  <<<<<<<<<<<<<<<<< THIS WAS THE PROBLEM. APPARENTLY CANT HAVE THIS.?>?????
pub struct Lookup {
    features: Vec<ExtractedFeature>,
    //index: BTreeMap<String, RoadSectionsByCarriageway>,
}

#[pymethods]
impl Lookup {

    // #[new]
    // pub fn new(py:Python) -> Self {
    //     py.run("print('dummy_conf3')", None, None).unwrap();
    //     Lookup {
    //         features: Vec::new(),
    //         //index: BTreeMap::new(),
    //     }
    // }

    /// (lat, lon, cwy, network_type, /)
    /// --
    ///
    /// A basic one-at-a-time lookup of the closest feature to the given lat/lon
    /// May be slow for bulk lookups
    //#[pyo3(text_signature = "($self, lat, lon, carriageways, network_type, /)")]
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

    // pub fn coordinate_from_road_slk(
    //     &self,
    //     road: &str,
    //     slk: f64,
    //     carriageways: u8,
    //     py: Python,
    // ) -> PyResult<PyObject> {
    //     let list_of_lists:Vec<PyObject> = self
    //         .index
    //         .get(road)
    //         .unwrap()
    //         .iter_matching_carriageways(carriageways)
    //         .filter_map(|(_cwy, index_range)| {
    //             let list_of_points:Vec<PyObject> = self
    //             .features[index_range]
    //             .into_iter()
    //             .filter_map(|feature|{
    //                 if feature.properties.slk_from <= slk && slk <= feature.properties.slk_to {
    //                     let fraction = (slk - feature.properties.slk_from) / (feature.properties.slk_to - feature.properties.slk_from);
    //                     match feature.geometry.0.line_interpolate_point(fraction){
    //                         Some(coordinate) => Some(PyTuple::new(py, &[coordinate.x().to_object(py), coordinate.y().to_object(py)]).to_object(py)),
    //                         None=>None
    //                     }
    //                 }else{
    //                     None
    //                 }
    //             })
    //             .collect();
    //             Some(PyList::new(py, list_of_points).to_object(py))
    //         }).collect();
    //     Ok(PyList::new(py, list_of_lists).into())
    // }

    // pub fn to_binary(&self, py: Python) -> PyResult<PyObject> {
    //     let encoded = bincode::serialize(&self).unwrap();
    //     let result;
    //     result = PyBytes::new_with(py, encoded.len(), |buffer| {
    //         buffer.copy_from_slice(&encoded);
    //         Ok(())
    //     });
    //     let result = result.unwrap();
    //     Ok(result.to_object(py))
    // }

    // #[staticmethod]
    // pub fn from_binary(input: &PyBytes) -> PyResult<Self> {
    //     let lookup:Self = bincode::deserialize(input.as_bytes()).unwrap();
    //     Ok(lookup)
    // }

    #[staticmethod]
    //#[pyo3(text_signature = "(python_dictionary,/)")]
    pub fn from_dict(python_dictionary: &PyDict) -> PyResult<Self> {
        // py.run("print('extracting features key')", None, None)?;

        let mut features: Vec<ExtractedFeature> = python_dictionary
            .get_item("features")
            .unwrap()
            .extract::<Vec<&PyAny>>()?
            .into_iter()
            .map(|feature| feature.extract::<ExtractedFeature>().unwrap())
            .collect();

        // let python_dictionary:&PyDict = match python_dictionary.extract(){
        //     Ok(python_dictionary) => python_dictionary,
        //     Err(_) => return Err(PyValueError::new_err("Function takes dictionary as argument"))
        // };

        // let features = match python_dictionary.get_item("features"){
        //     Some(features) => features,
        //     None=> return Err(PyValueError::new_err("No features found in dictionary"))
        // };

        // py.run("print('extracting list')", None, None)?;

        // // let features =  match features.cast_as::<PyList>(){
        // //     Ok(features) => features,
        // //     Err(_) => return Err(PyValueError::new_err("Features must be a list"))
        // // };

        // py.run("print('Extracting ExtractedFeature')", None, None)?;
        // let features = match features.extract::<Vec<ExtractedFeature>>(){
        //     Ok(features) => features,
        //     Err(_) => return Err(PyValueError::new_err("`features` must be a list of ExtractedFeature"))
        // };

        
        // Notes:
        // - .sort() is stable; order of equal elements is preserved.
        // - The order of input elements is currently unknown.
        // - the sorted Ord trait of ExtractedFeature currently does not look at slk or true.
        // - we do still need to sort by road and cwy thats what this does;
        
        //features.sort(); 

        // let index = Self::build_index(&features);
        //let index = BTreeMap::new();
        
        Ok(Self {
            features,
            //index
        })
    }


    // pub fn get_index(&self, py:Python)-> PyResult<PyObject>{
    //     let index_dict = PyDict::new(py);
    //     for (key, value) in &self.index {
    //         index_dict.set_item(key, value.to_object(py))?;
    //     }
    //     Ok(index_dict.to_object(py))
    // }

    
        
}


// impl Lookup{
//     fn build_index(features: & Vec<ExtractedFeature>) -> BTreeMap<String, RoadSectionsByCarriageway> {
//         let mut index = BTreeMap::new();
        
        

//         // result.insert("".into(), RoadSectionsByCarriageway::new(Some((1,2)), None, None));
//         // return result;

//         let mut current_slice_start = 0;

//         for i in 1..20 {
//             let a_feature = &features[i-1];
//             let b_feature = &features[i];
//             let b_feature_is_new_road = a_feature.properties.road != b_feature.properties.road;
//             let b_feature_is_new_cwy  = a_feature.properties.cwy  != b_feature.properties.cwy;
            
//             if b_feature_is_new_road || b_feature_is_new_cwy {
//                 match index.entry(b_feature.properties.road.clone()) {
//                     Entry::Vacant(e) => {
//                         e.insert(RoadSectionsByCarriageway::new_from_cwy(
//                             &a_feature.properties.cwy,
//                             (current_slice_start, i),
//                         ));
//                     }
//                     Entry::Occupied(mut e) => {
//                         e.insert(
//                             e
//                             .get()
//                             .with_updated_cwy(
//                                 &a_feature.properties.cwy,
//                                  (current_slice_start, i)
//                             ),
//                         );
//                     }
//                 }
//                 current_slice_start = i;
//             }
//         }
//         index
//     }
// }