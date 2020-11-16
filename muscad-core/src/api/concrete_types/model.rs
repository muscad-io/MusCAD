use super::*;

#[derive(Debug)]
pub struct Model<T>(pub sealed::Model<T>);

impl<T: Float> Model<T> {
    pub fn new() -> Self {
        Self(sealed::Model::new())
    }

    pub fn add_face<V, P>(&mut self, pts: &V) -> Result<Face<T>, Error>
    where
        V: AsRef<[P]>,
        P: AsRef<Point3d<T>> + sealed::AsV3d<T>,
    {
        sealed::check::face_pts_len(&pts)?;

        let f = self.0.add_face_unchecked(&pts);

        Ok(Face::from_rc(&f))
    }
}
