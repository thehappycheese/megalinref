
use bincode;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyBytes};
use serde::{Serialize, Deserialize};
use rstar::RTree;

use crate::util::extracted::ExtractedFeature;
use crate::util::pyobject_linestring::MyLineString;


#[derive(Serialize, Deserialize, Debug, Clone)]
#[pyclass]
pub struct SLKLookup{
    features:Vec<ExtractedFeature>,
    rtree:RTree<MyLineString>
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
        //panic!("AFTER FEATURE COLLECT");
        Ok(Self{
            features:features
        })
    }

    pub fn lookup(&self, index:usize) -> PyResult<String>{
        Ok(format!("{}", self.features[index].properties.road))
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
        Ok(Self{
            features:bincode::deserialize(input.as_bytes()).unwrap()
        })
    }
}