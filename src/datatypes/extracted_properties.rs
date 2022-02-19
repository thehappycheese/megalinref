
use pyo3::types::PyDict;

use pyo3::prelude::*;
//use serde::{Deserialize, Serialize};
use super::{Cwy, NetworkType};

/// Used to store the properties associated with each LineString in the road network.
/// 
/// NOTE: The `FromPyObject` and `ToPyObject` traits are implemented with the hard-coded dict key names:
/// 
/// - `"ROAD"`
/// - `"CWY"`
/// - `"START_SLK"`
/// - `"END_SLK"`
/// - `"START_TRUE_DIST"`
/// - `"END_TRUE_DIST"`
/// - `"NETWORK_TYPE"`
/// 
/// The python part of this package is responsible for checking that data downloaded 
/// from the source uses these same property names.
/// 
#[derive(
    // Serialize, Deserialize,
    Clone,
    // PartialEq, PartialOrd
)]
pub struct ExtractedProperties{
    pub road:String,
    pub cwy:Cwy,
    pub slk_from:f64,
    pub slk_to:f64,
    pub true_from:f64,
    pub true_to:f64,
    pub network_type:NetworkType,
}

/// This cannot be derived because some members only implement PartialEq and PartialOrd. 
/// but Eq is required so that we are allowed to implement Ord.
/// This basically amounts to an assurance to the compiler, and its pretty hard to understand why it is required.

//impl Eq for ExtractedProperties{}


/// Ord only depends on `road` and `cwy`.
/// We need to be careful that we do not rely on road segments being sorted by `slk_from` or `true_from`.
/// If that is needed it must be introduced here.
 
// impl Ord for ExtractedProperties {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         match self.road.cmp(&other.road){
//             std::cmp::Ordering::Equal => self.cwy.cmp(&other.cwy),
//             x => x
//         }
//     }
// }

impl<'a> FromPyObject<'a> for ExtractedProperties{
    fn extract(dict: &'a PyAny) -> PyResult<Self> {
        let dict:&PyDict = match dict.extract(){
            Ok(dict) => dict,
            Err(_) => return Err(pyo3::exceptions::PyException::new_err(
                "'properties' must be of type dict"
            )),
        };
        
        Ok(Self{
            road:         "NONE".to_string(),
            cwy:          Cwy::Left,
            slk_from:     0.0,
            slk_to:       0.0,
            true_from:    0.0,
            true_to:      0.0,
            network_type: NetworkType::State_Road,
        })

        // Ok(Self{
            // road:         dict.get_item("ROAD")            .unwrap().extract::<String>()?,
            // cwy:          dict.get_item("CWY")             .unwrap().extract::<Cwy>()?,
            // slk_from:     dict.get_item("START_SLK")       .unwrap().extract::<f64>()?,
            // slk_to:       dict.get_item("END_SLK")         .unwrap().extract::<f64>()?,
            // true_from:    dict.get_item("START_TRUE_DIST") .unwrap().extract::<f64>()?,
            // true_to:      dict.get_item("END_TRUE_DIST")   .unwrap().extract::<f64>()?,
            // network_type: dict.get_item("NETWORK_TYPE")    .unwrap().extract::<NetworkType>()?,
        // })
    }
}

// impl ToPyObject for ExtractedProperties {
//     fn to_object(&self, py: Python) -> PyObject {
//         let dict = PyDict::new(py);
//         dict.set_item("ROAD", self.road.to_object(py)).unwrap();
//         dict.set_item("CWY", self.cwy.to_object(py)).unwrap();
//         dict.set_item("START_SLK", self.slk_from.to_object(py)).unwrap();
//         dict.set_item("END_SLK", self.slk_to.to_object(py)).unwrap();
//         dict.set_item("START_TRUE_DIST", self.true_from.to_object(py)).unwrap();
//         dict.set_item("END_TRUE_DIST", self.true_to.to_object(py)).unwrap();
//         dict.set_item("NETWORK_TYPE", self.network_type.to_object(py)).unwrap();
//         dict.to_object(py)
//     }
// }