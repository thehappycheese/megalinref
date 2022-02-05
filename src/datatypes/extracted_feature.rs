

use std::fmt;
use std::sync::Arc;

use serde::de::{MapAccess, Visitor};
use serde::{Serializer, de, Deserializer};
use pyo3::prelude::*;
use pyo3::types::{PyDict};
use serde::ser::SerializeStruct;
use serde::{Serialize, Deserialize};

use super::{ExtractedLineString, ExtractedProperties};

use rstar::{RTreeObject, AABB, PointDistance};





#[derive(Serialize, Deserialize)]
pub struct ExtractedFeature {
    pub index:usize,
    pub properties:ExtractedProperties,
    pub geometry:ExtractedLineString
}

impl ExtractedFeature{
    // extract from pyobject with index set
    pub fn from_pyobject_with_index(obj: &PyAny, index:usize) -> PyResult<Self> {
        let dict = obj.extract::<&PyDict>()?;
        let properties = dict.get_item("properties").unwrap().extract::<ExtractedProperties>()?;
        let geometry = dict.get_item("geometry").unwrap().extract::<ExtractedLineString>()?;
        Ok(Self{
            index,
            properties,
            geometry
        })
    }
}

impl<'a> FromPyObject<'a> for ExtractedFeature{
    fn extract(obj: &'a PyAny) -> PyResult<Self> {
        let dict = obj.extract::<&PyDict>()?;
        let properties = dict.get_item("properties").unwrap().extract::<ExtractedProperties>()?;
        let geometry = dict.get_item("geometry").unwrap().extract::<ExtractedLineString>()?;
        Ok(Self{
            index:0,
            properties,
            geometry
        })
    }
}

pub struct ArcBoxExtractedFeature(pub Arc<Box<ExtractedFeature>>);


impl Serialize for ArcBoxExtractedFeature{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ExtractedFeature", 2)?;
        state.serialize_field("properties", &self.0.as_ref().properties)?;
        state.serialize_field("geometry", &self.0.as_ref().geometry)?;
        state.end()
    }
}


// this code was blasted out by copilot
// Im sure its not necessary, we should be able to delegate most of this complexity to the derived deserialize impl for ExtractedFeature
impl<'de> Deserialize<'de> for ArcBoxExtractedFeature{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Properties,
            Geometry
        }

        struct ExtractedFeatureVisitor;

        impl<'de> Visitor<'de> for ExtractedFeatureVisitor {
            type Value = ArcBoxExtractedFeature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ExtractedFeature")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut properties = None;
                let mut geometry = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Properties => {
                            if properties.is_some() {
                                return Err(de::Error::duplicate_field("properties"));
                            }
                            properties = Some(map.next_value()?);
                        }
                        Field::Geometry => {
                            if geometry.is_some() {
                                return Err(de::Error::duplicate_field("geometry"));
                            }
                            geometry = Some(map.next_value()?);
                        }
                    }
                }

                let properties = properties.ok_or_else(|| de::Error::missing_field("properties"))?;
                let geometry = geometry.ok_or_else(|| de::Error::missing_field("geometry"))?;

                Ok(ArcBoxExtractedFeature(Arc::new(Box::new(ExtractedFeature{
                    index:0,
                    properties,
                    geometry
                }))))
            }
        }

        const FIELDS: &[&str] = &["properties", "geometry"];
        deserializer.deserialize_struct("ExtractedFeature", FIELDS, ExtractedFeatureVisitor)

    }
}

impl RTreeObject for ArcBoxExtractedFeature {
    type Envelope = AABB<geo_types::Point<f64>>;
    fn envelope(&self) -> Self::Envelope {
        self.0.geometry.0.envelope().into()
    }
}

impl PointDistance for ArcBoxExtractedFeature {
    fn distance_2(&self, point: &geo::Point<f64>) -> f64 {
        self.0.geometry.0.distance_2(point)
    }
}