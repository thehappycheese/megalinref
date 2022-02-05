
use std::sync::Arc;

use bincode;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyBytes, PyList};
use rstar::{RTree, RTreeParams, RStarInsertionStrategy};



use crate::datatypes::{
    ExtractedFeature,
    ArcBoxExtractedFeature,
};

struct LargeNodeParameters;
impl RTreeParams for LargeNodeParameters {
    const MIN_SIZE:          usize = 200;
    const MAX_SIZE:          usize = 2000;
    const REINSERTION_COUNT: usize = 2;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}
type LargeNodeRTree<T> = RTree<T, LargeNodeParameters>;



#[pyclass]
pub struct SLKLookup{
    features:Vec<ArcBoxExtractedFeature>,
    spatial_index:LargeNodeRTree<ArcBoxExtractedFeature>,
}

#[pymethods]
impl SLKLookup{
    #[new]
    pub fn new(python_dictionary:&PyDict) -> PyResult<Self>{
        let features:Vec<ArcBoxExtractedFeature> = python_dictionary
        .get_item("features")
        .unwrap()
        .extract::<Vec<&PyAny>>()?
        .into_iter()
        .enumerate()
        .map(|(index, item)|{
            println!("DO {:?}", item);
            ArcBoxExtractedFeature(Arc::new(Box::new(ExtractedFeature::from_pyobject_with_index(item, index).unwrap())))
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
            .map(|ArcBoxExtractedFeature(feature)| feature.index)
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
        Self::from_features(bincode::deserialize(input.as_bytes()).unwrap())
    }

    
}

// By creating a separate impl block we escape the requirements of the #[pyclass] / #[pymethods] macros
impl SLKLookup{
    fn from_features(features:Vec<ArcBoxExtractedFeature>) -> PyResult<Self> {
        let spatial_index = LargeNodeRTree::bulk_load_with_params(
            (&features)
            .iter()
            .map(|feat| ArcBoxExtractedFeature(feat.0.clone()))
            .collect()
        );
        Ok(Self{
            features,
            spatial_index
        })
    }
}