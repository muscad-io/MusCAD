use super::*;

pub use data::*;

mod analyse;
mod connect;
mod data;
mod generate;
mod outer_loop;
mod segments;

impl<T: Float> BooleanSolver<T> {
    pub fn generate_data_for_face(&mut self, d: &mut Vec<BrepData<T>>, f: FaceRef<T>) {
        let key = f.to_key();

        match self.intersections.face_face_pt_map.get(&key) {
            Some(_) => {
                let mut data = self.analyse_brep_configuration(f);

                self.generate_new_face(&mut data);

                d.push(data)
            }
            None => d.push(BrepData::unchanged(key.to_inner())),
        }
    }
}
