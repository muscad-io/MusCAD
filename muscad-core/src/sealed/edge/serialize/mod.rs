use super::*;

use serde::ser::{Serialize, SerializeStruct, Serializer};

impl<T: Float + Serialize> Serialize for Edge<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut res = serializer.serialize_struct("Edge", 1)?;

        res.serialize_field(
            "pts",
            &[self.to_p1(), self.to_p2()],
        )?;

        res.end()
    }
}
