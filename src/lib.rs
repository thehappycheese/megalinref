//#![feature(specialization)]

extern crate pyo3;

use pyo3::{prelude::*, types::PyList, types::PyDict};

mod lookup;
mod datatypes;
mod util;

use datatypes::{
    Cwy,
    NetworkType
};
use lookup::Lookup;


/// My Module test documentation
#[pymodule]
fn megalinref(py: Python, module: &PyModule) -> PyResult<()> {

    // Cwy enum represented as dict
    module.add("Cwy", PyDict::from_sequence(
            py, 
            PyList::new(py, &vec![
                ("L", Cwy::Left   as u8),
                ("S", Cwy::Single as u8),
                ("R", Cwy::Right  as u8),

                ("LS",  (Cwy::Left  as u8) | (Cwy::Single as u8)),
                ("RS",  (Cwy::Right as u8) | (Cwy::Single as u8)),
                ("LR",  (Cwy::Left  as u8) | (Cwy::Right  as u8)),
                ("LRS", (Cwy::Left  as u8) | (Cwy::Single as u8) | (Cwy::Right as u8)),
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
                ("Main Roads Controlled Path", NetworkType::Main_Roads_Controlled_Path as u8),
                ("Proposed Road",              NetworkType::Proposed_Road as u8),
                ("Crossover",                  NetworkType::Crossover as u8),
            ]).to_object(py)
        ).unwrap()
    )?;

    module.add_class::<Lookup>()?;

    Ok(())
}