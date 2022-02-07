//#![feature(specialization)]

extern crate pyo3;

use pyo3::{prelude::*, types::PyList, types::PyDict};

mod slk_from_lat_lon;
mod datatypes;
mod util;

use datatypes::{
    Cwy,
    NetworkType
};
use slk_from_lat_lon::SLKLookup;


/// My Module test documentation
#[pymodule]
fn megalinref(py: Python, module: &PyModule) -> PyResult<()> {

    // Cwy enum represented as dict
    module.add("Cwy", PyDict::from_sequence(
            py, 
            PyList::new(py, &vec![
                ("L", Cwy::L as u8),
                ("S", Cwy::S as u8),
                ("R", Cwy::R as u8),
            ]).to_object(py)
        ).unwrap()
    )?;
    
    // NetworkType enum represented as dict
    module.add("NetworkType", PyDict::from_sequence(
            py, 
            PyList::new(py, &vec![
                ("State Road",                 NetworkType::State_Road as u8),
                ("Local Road",                 NetworkType::Local_Road as u8),
                ("Miscellaneous Road",         NetworkType::Miscellaneous_Road as u8),
                ("Main Roads Controlled_Path", NetworkType::Main_Roads_Controlled_Path as u8),
                ("Proposed Road",              NetworkType::Proposed_Road as u8),
                ("Crossover",                  NetworkType::Crossover as u8),
            ]).to_object(py)
        ).unwrap()
    )?;

    module.add_class::<SLKLookup>()?;

    Ok(())
}