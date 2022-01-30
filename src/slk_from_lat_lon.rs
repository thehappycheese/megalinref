

use geo::{LineString};


use pyo3::prelude::*;
use pyo3::types::{PyDict};
use serde::{Serialize, Deserialize};

use crate::enums::{Cwy, NetworkType};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MyLineString(LineString<f64>);

impl<'a> FromPyObject<'a> for MyLineString{
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        let linestring_dict = obj.extract::<&PyDict>()?;
        let coords = linestring_dict
            .get_item("coordinates")
            .unwrap()
            .extract::<Vec<&PyAny>>()?;
        
        let coords:Vec<(f64, f64)> = coords.into_iter().map(|item|{
            let list = item.extract::<Vec<f64>>().unwrap();
            (list[0], list[1])
        }).collect();
        let result  = MyLineString(LineString::from(coords));
        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[pyclass]
pub struct SLKLookup{
    features:Vec<ExtractedFeature>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedProperties{
    road:String,
    cwy:Cwy,
    slk_from:f64,
    slk_to:f64,
    true_from:f64,
    true_to:f64,
    network_type:NetworkType,
}

impl<'a> FromPyObject<'a> for ExtractedProperties{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let ob = ob.cast_as::<PyDict>()?;
        
        Ok(Self{
            road:         ob.get_item("ROAD")            .unwrap().extract::<String>()?,
            cwy:          ob.get_item("CWY")             .unwrap().extract::<Cwy>()?,
            slk_from:     ob.get_item("START_SLK")       .unwrap().extract::<f64>()?,
            slk_to:       ob.get_item("END_SLK")         .unwrap().extract::<f64>()?,
            true_from:    ob.get_item("START_TRUE_DIST") .unwrap().extract::<f64>()?,
            true_to:      ob.get_item("END_TRUE_DIST")   .unwrap().extract::<f64>()?,
            network_type: ob.get_item("NETWORK_TYPE")    .unwrap().extract::<NetworkType>()?,
        })
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtractedFeature {
    properties:ExtractedProperties,
    geometry:MyLineString
}

impl<'a> FromPyObject<'a> for ExtractedFeature{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let dict = ob.extract::<&PyDict>()?;
        let properties = dict.get_item("properties").unwrap().extract::<ExtractedProperties>()?;
        let geometry = dict.get_item("geometry").unwrap().extract::<MyLineString>()?;
        Ok(Self{
            properties,
            geometry
        })
    }
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
}