use super::*;

use serde::ser::{Serialize, SerializeStruct, Serializer};

impl<T: Float + Serialize> Serialize for Model<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let pts = self.to_points();
        let mut triangles = vec![];
        let mut lines = vec![];

        let to_index = |rc: Rc<Vertex<T>>| self.get_point_index(rc.as_ref()).unwrap();

        self.for_each_face(|f| {
            let pool_indices: Vec<_> = f.vertices_iter().map(to_index).collect();

            triangles.extend(
                poly2d::triangulate_unchecked(&f.to_positions2d())
                    .into_iter()
                    .map(|i| pool_indices[i]),
            );

            f.for_each_edge(|e| {
                lines.push(to_index(e.to_v1()));
                lines.push(to_index(e.to_v2()));
            });
        });

        let mut res = serializer.serialize_struct("Model", 3)?;

        res.serialize_field("pts", &pts)?;
        res.serialize_field("triangles", &triangles)?;
        res.serialize_field("lines", &lines)?;

        res.end()
    }
}
