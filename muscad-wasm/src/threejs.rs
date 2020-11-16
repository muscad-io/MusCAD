#[allow(unused)]
#[derive(Debug, Serialize)]
pub struct ThreeData {
    pub pts: Vec<[f64; 3]>,
    pub triangles: Vec<usize>,
    pub lines: Vec<usize>,
}

pub fn serialize_model(m: &muscad::Model<f64>) -> String {
    let vertices = m.vertex_pool.get_verts();

    let (triangles, lines) = m.with_faces(|faces| {
        let mut face_indices = vec![];
        let mut lines = vec![];
        for f in faces {
            let f = f.borrow();
            let pool_indices = f
                .vertices_iter()
                .map(|v| m.vertex_pool.get_index(&v).unwrap())
                .collect::<Vec<_>>();

            let pts2d = f
                .vertices_iter()
                .map(|v| (f.project)(&v.position))
                .collect::<Vec<_>>();

            let indices = muscad::geom::vector2d::triangulate2d(&pts2d)
                .into_iter()
                .map(|i| pool_indices[i])
                .collect::<Vec<_>>();

            face_indices.extend(indices);

            for e in f.edges_iter() {
                let (v1, v2) = e.borrow().segment();
                let (i1, i2) = (
                    m.vertex_pool.get_index(&v1).unwrap(),
                    m.vertex_pool.get_index(&v2).unwrap(),
                );
                lines.push(i1);
                lines.push(i2);
            }
        }

        (face_indices, lines)
    });

    let d = ThreeData {
        pts: vertices,
        triangles,
        lines,
    };

    serde_json::to_string(&d).unwrap()
}
