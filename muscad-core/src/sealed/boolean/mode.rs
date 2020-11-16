#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub enum BooleanMode {
    All,
    AMinusB,
    BMinusA,
    Union,
    Intersection,
    SymmetricDifference,
}
