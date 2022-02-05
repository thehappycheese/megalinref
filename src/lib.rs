//#![feature(specialization)]

extern crate pyo3;

use pyo3::{prelude::*, types::PyList, types::PyDict};

mod slk_from_lat_lon;
mod datatypes;

use datatypes::{
    Cwy,
    NetworkType
};
use slk_from_lat_lon::SLKLookup;



#[pymodule]
fn megalinref(_py: Python, m: &PyModule) -> PyResult<()> {

    let cwy_dict = PyDict::from_sequence(
        _py, 
        PyList::new(_py, &vec![
            ("L", Cwy::L),
            ("S", Cwy::S),
            ("R", Cwy::R),
        ]).to_object(_py)
    ).unwrap();
    
    let network_type_dict = PyDict::from_sequence(
        _py, 
        PyList::new(_py, &vec![
            ("State Road",                 NetworkType::State_Road),
            ("Local Road",                 NetworkType::Local_Road),
            ("Miscellaneous Road",         NetworkType::Miscellaneous_Road),
            ("Main Roads Controlled_Path", NetworkType::Main_Roads_Controlled_Path),
            ("Proposed Road",              NetworkType::Proposed_Road),
            ("Crossover",                  NetworkType::Crossover),
        ]).to_object(_py)
    ).unwrap();
    

    m.add("CWY", cwy_dict)?;
    m.add("NETWORK_TYPE", network_type_dict)?;
    m.add_class::<SLKLookup>()?;

    Ok(())
}