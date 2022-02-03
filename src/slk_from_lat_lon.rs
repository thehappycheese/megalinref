
use bincode;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyBytes, PyList};
use rstar::{RTree};


use crate::util::extracted::ExtractedFeature;
use crate::util::pyobject_linestring::MyLineString;


#[pyclass]
pub struct SLKLookup{
    features:Vec<ExtractedFeature>,
    spatial_index:RTree<MyLineString>,
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
        .map(|item|{
            println!("DO {:?}", item);
            item.extract().unwrap()
        }).collect();
        Self::from_features(features)
        
    }

    pub fn lookup(&self, lat:f64, lon:f64, dist:f64, py:Python) -> PyResult<PyObject>{
        let result:Vec<usize> = self
            .spatial_index
            .locate_within_distance(
                [lat,lon].into(),
                dist
            )
            .map(|MyLineString(_, index)| *index)
            .collect();
        Ok(PyList::new(py, &result).to_object(py))
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

    // pub fn to_binary(&self, py:Python) -> PyResult<PyObject>{
    //     let slc = bincode::serialize(&self.features).unwrap().as_slice();
    //     Ok(
    //         PyBytes::new(
    //             py,
    //             slc
    //         )
    //     )
    // }

    #[staticmethod]
    pub fn from_binary(input:&PyBytes) -> PyResult<Self>{
        Self::from_features(
            bincode::deserialize(input.as_bytes()).unwrap()
        )
    }

    #[staticmethod]
    fn from_features(features:Vec<ExtractedFeature>) -> PyResult<Self> {
        let spatial_index = RTree::bulk_load(
            (&features)
            .iter()
            .enumerate()
            .map(|(index, ExtractedFeature{geometry:MyLineString(ls,_), ..})| MyLineString(ls.clone(), index))
            .collect()
        );
        Ok(Self{features, spatial_index})
    }
}