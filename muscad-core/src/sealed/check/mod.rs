use super::*;

pub fn face_pts_len<U, V>(pts: &V) -> Result<(), OperationError<NotEnoughPoints>>
where
    V: AsRef<[U]>,
{
    if pts.as_ref().len() < 3 {
        Err(OperationError::new())
    } else {
        Ok(())
    }
}
