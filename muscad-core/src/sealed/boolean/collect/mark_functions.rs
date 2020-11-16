use super::*;

type MarkFunction<T> = fn(&mut FaceData<T>, &Classification, bool) -> ();

pub fn get_mark_function<T: Float>(mode: &BooleanMode) -> MarkFunction<T> {
    match mode {
        BooleanMode::All => _all,
        BooleanMode::AMinusB => _a_minus_b,
        BooleanMode::BMinusA => _b_minus_a,
        BooleanMode::Union => _union,
        BooleanMode::Intersection => _intersection,
        BooleanMode::SymmetricDifference => _symmetric_difference,
    }
}

fn _all<T: Float>(f: &mut FaceData<T>, _c: &Classification, _face_from_a: bool) {
    f.skip = false;
    f.rev = false;
}

fn _a_minus_b<T: Float>(f: &mut FaceData<T>, c: &Classification, face_from_a: bool) {
    if face_from_a {
        if *c == Classification::Out {
            f.skip = false;
            f.rev = false;
        }
    } else {
        if *c == Classification::In {
            f.skip = false;
            f.rev = true;
        }
    }
}

fn _b_minus_a<T: Float>(f: &mut FaceData<T>, c: &Classification, face_from_a: bool) {
    if face_from_a {
        if *c == Classification::In {
            f.skip = false;
            f.rev = true;
        }
    } else {
        if *c == Classification::Out {
            f.skip = false;
            f.rev = false;
        }
    }
}

fn _symmetric_difference<T: Float>(f: &mut FaceData<T>, c: &Classification, _face_from_a: bool) {
    if *c == Classification::In {
        f.skip = false;
        f.rev = true;
    } else if *c == Classification::Out {
        f.skip = false;
        f.rev = false;
    }
}

fn _intersection<T: Float>(f: &mut FaceData<T>, c: &Classification, _face_from_a: bool) {
    if *c == Classification::In {
        f.skip = false;
        f.rev = false;
    }
}

fn _union<T: Float>(f: &mut FaceData<T>, c: &Classification, _face_from_a: bool) {
    if *c == Classification::Out {
        f.skip = false;
        f.rev = false;
    }
}
