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


/// My Module test documentation
#[pymodule]
fn megalinref(py: Python, module: &PyModule) -> PyResult<()> {

    // Cwy enum represented as dict
    module.add("Cwy", PyDict::from_sequence(
            py, 
            PyList::new(py, &vec![
                ("L", Cwy::L),
                ("S", Cwy::S),
                ("R", Cwy::R),
            ]).to_object(py)
        ).unwrap()
    )?;
    
    // NetworkType enum represented as dict
    module.add("NetworkType", PyDict::from_sequence(
            py, 
            PyList::new(py, &vec![
                ("State Road",                 NetworkType::State_Road),
                ("Local Road",                 NetworkType::Local_Road),
                ("Miscellaneous Road",         NetworkType::Miscellaneous_Road),
                ("Main Roads Controlled_Path", NetworkType::Main_Roads_Controlled_Path),
                ("Proposed Road",              NetworkType::Proposed_Road),
                ("Crossover",                  NetworkType::Crossover),
            ]).to_object(py)
        ).unwrap()
    )?;

    module.add_class::<SLKLookup>()?;

    Ok(())
}