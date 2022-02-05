
use bincode;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyBytes, PyList};

use rstar::{RTree, RTreeParams, RStarInsertionStrategy};
use rstar::primitives::{GeomWithData, Line};



use crate::datatypes::{
    ExtractedFeature,
};

struct LargeNodeParameters;
impl RTreeParams for LargeNodeParameters {
    const MIN_SIZE:          usize = 4;
    const MAX_SIZE:          usize = 10;
    const REINSERTION_COUNT: usize = 3;
    type DefaultInsertionStrategy = RStarInsertionStrategy;
}
type LargeNodeRTree<T> = RTree<T, LargeNodeParameters>;



type SpatialIndexLineSegment = GeomWithData<Line<[f64;2]>, usize>;



#[pyclass]
pub struct SLKLookup{
    features:Vec<ExtractedFeature>,
    spatial_index:LargeNodeRTree<SpatialIndexLineSegment>,
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
        .enumerate()
        .map(|(index, item)|{
            println!("DO {:?}", item);
            ExtractedFeature::from_pyobject_with_index(item, index).unwrap()
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
            .map(|item| item.data)
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

    #[staticmethod]
    pub fn from_binary(input:&PyBytes) -> PyResult<Self>{
        Self::from_features(bincode::deserialize(input.as_bytes()).unwrap())
    }

    
}

// By creating a separate impl block we escape the requirements of the #[pyclass] / #[pymethods] macros



impl SLKLookup{
    /// TODO: this function basically exists to build the RTree, but it turns out we can serialize the RTree
    ///       so we should only do this for new trees.
    fn from_features(features:Vec<ExtractedFeature>) -> PyResult<Self> {
        
        let spatial_index = LargeNodeRTree::bulk_load_with_params(
            (&features)
            .iter()
            .enumerate()
            .flat_map(|(index, feat)| {
                // extract line segments from feat.geometry
                let line_segments:Vec<SpatialIndexLineSegment> = feat
                    .geometry
                    .0
                    .lines()
                    .map(|geo_line| {
                        let start = geo_line.start;
                        let end = geo_line.end;
                        SpatialIndexLineSegment::new(
                            Line::new(
                                [start.x, start.y],
                                [end.x, end.y]
                            ), 
                            index
                        )
                    })
                    .collect();
                line_segments
            })
            .collect()
        );
        Ok(Self{
            features,
            spatial_index
        })
    }
}