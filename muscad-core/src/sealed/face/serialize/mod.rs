use super::*;

use serde::ser::{Serialize, SerializeStruct, Serializer};

impl<T: Float + Serialize> Serialize for Face<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut pts = vec![];

        self.for_each_edge(|e| pts.push(e.to_p1()));
        let mut res = serializer.serialize_struct("Face", 1)?;

        res.serialize_field("pts", &pts)?;

        res.end()
    }
}
