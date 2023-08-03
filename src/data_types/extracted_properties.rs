
use pyo3::types::PyDict;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
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
    Serialize, Deserialize,
    Clone,
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

/// Eq is required so that we are allowed to implement Ord.
/// This cannot be derived because some members
/// only implement PartialEq and PartialOrd (f64).
impl Eq for ExtractedProperties{}


/// This implementation is consistent with Ord
/// To guarantee that datatypes work nicely with other
/// We must ensure a.cmp(b) == Ordering::Equal if and only if a.eq(b)
impl PartialEq for ExtractedProperties{
    fn eq(&self, other: &Self) -> bool {
        self.road == other.road &&
        self.cwy == other.cwy
    }
}


/// Ord only depends on `road` and `cwy`.
/// We need to be careful that we do not rely on road segments being sorted by `slk_from` or `true_from`.
/// If that is needed it must be introduced here.
impl Ord for ExtractedProperties{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.road.cmp(&other.road){
            std::cmp::Ordering::Equal => self.cwy.cmp(&other.cwy),
            x => x
        }
    }
}

impl PartialOrd for ExtractedProperties {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> FromPyObject<'a> for ExtractedProperties{
    fn extract(dict: &'a PyAny) -> PyResult<Self> {
        let dict:&PyDict = match dict.extract(){
            Ok(dict) => dict,
            Err(_) => return Err(pyo3::exceptions::PyException::new_err(
                "'properties' must be of type dict"
            )),
        };

        macro_rules! try_extract_from_dict {
            ($key:expr, $typ:ty) => {
                match dict.get_item($key){
                    Some(pyany) => match pyany.extract::<$typ>() {
                        Ok(value) => value,
                        Err(_) => return Err(pyo3::exceptions::PyException::new_err(
                            format!("'{}' must be of type {}", $key , stringify!($typ))
                        )),
                    },
                    None => return Err(pyo3::exceptions::PyException::new_err(
                        format!("Item '{}' missing from 'properties'", $key)
                    )),
                }
            }
        }
        
        Ok(Self{
            road         : try_extract_from_dict!("ROAD",            String      ),
            cwy          : try_extract_from_dict!("CWY",             Cwy         ),
            network_type : try_extract_from_dict!("NETWORK_TYPE",    NetworkType ),
            slk_from     : try_extract_from_dict!("START_SLK",       f64         ),
            slk_to       : try_extract_from_dict!("END_SLK",         f64         ),
            true_from    : try_extract_from_dict!("START_TRUE_DIST", f64         ),
            true_to      : try_extract_from_dict!("END_TRUE_DIST",   f64         ),
        })

    }
}

impl ToPyObject for ExtractedProperties {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("ROAD", self.road.to_object(py)).unwrap();
        dict.set_item("CWY", self.cwy.to_object(py)).unwrap();
        dict.set_item("START_SLK", self.slk_from.to_object(py)).unwrap();
        dict.set_item("END_SLK", self.slk_to.to_object(py)).unwrap();
        dict.set_item("START_TRUE_DIST", self.true_from.to_object(py)).unwrap();
        dict.set_item("END_TRUE_DIST", self.true_to.to_object(py)).unwrap();
        dict.set_item("NETWORK_TYPE", self.network_type.to_object(py)).unwrap();
        dict.to_object(py)
    }
}