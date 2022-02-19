use pyo3::{
    types::{PyDict, PyTuple},
    PyObject, Python, ToPyObject,
};
// use serde::{Deserialize, Serialize};

use super::Cwy;

//#[derive(Serialize, Deserialize)]
pub struct RoadSectionsByCarriageway {
    left: Option<(usize, usize)>,
    single: Option<(usize, usize)>,
    right: Option<(usize, usize)>,
}

impl RoadSectionsByCarriageway {
    pub fn new(
        left:   Option<(usize, usize)>,
        single: Option<(usize, usize)>,
        right:  Option<(usize, usize)>,
    ) -> Self {
        Self {
            left,
            single,
            right,
        }
    }
    pub fn new_from_cwy(cwy: &Cwy, range: (usize, usize)) -> Self {
        match cwy {
            Cwy::Left   => Self::new(Some(range), None, None),
            Cwy::Right  => Self::new(None, Some(range), None),
            Cwy::Single => Self::new(None, None, Some(range)),
        }
    }
    pub fn with_updated_cwy(&self, cwy: &Cwy, range: (usize, usize)) -> Self {
        match cwy {
            Cwy::Left   => Self::new(Some(range), self.right, self.single),
            Cwy::Right  => Self::new(self.left, Some(range), self.single),
            Cwy::Single => Self::new(self.left, self.right, Some(range)),
        }
    }

    pub fn iter_matching_carriageways(
        &self,
        carriageways: u8,
    ) -> impl std::iter::Iterator<Item = (Cwy, std::ops::Range<usize>)> {
        [
            (Cwy::Left, self.left),
            (Cwy::Single, self.single),
            (Cwy::Right, self.right),
        ]
        .into_iter()
        .filter_map(move |(cwy, range)| match range {
            Some((range_start, range_end)) => match (cwy as u8) & carriageways {
                0 => None,
                _ => Some((cwy, range_start..range_end)),
            },
            None => None,
        })
    }
}
impl ToPyObject for RoadSectionsByCarriageway {
    fn to_object(&self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item(
            "left",
            match self.left {
                Some((start, end)) => {
                    Some(PyTuple::new(py, &[start.to_object(py), end.to_object(py)]).to_object(py))
                }
                None => Some(Python::None(py)),
            },
        )
        .unwrap();
        dict.set_item(
            "single",
            match self.single {
                Some((start, end)) => {
                    Some(PyTuple::new(py, &[start.to_object(py), end.to_object(py)]).to_object(py))
                }
                None => Some(Python::None(py)),
            },
        )
        .unwrap();
        dict.set_item(
            "right",
            match self.right {
                Some((start, end)) => {
                    Some(PyTuple::new(py, &[start.to_object(py), end.to_object(py)]).to_object(py))
                }
                None => Some(Python::None(py)),
            },
        )
        .unwrap();
        dict.to_object(py)
    }
}
