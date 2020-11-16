use super::*;

use muscad::{BooleanMode, BooleanSolver};

pub fn a_minus_b(a: &Model<S>, b: &Model<S>) -> Model<S> {
    let m = a.with_inner(|a| {
        b.with_inner(|b| {
            let mut solver = BooleanSolver::new();
            solver.mode = BooleanMode::AMinusB;

            solver.eval(&a.0, &b.0)
        })
    });

    Model::new(muscad::Model(m))
}

pub fn load(vm: &Thread) -> vm::Result<ExternModule> {
    ExternModule::new(
        vm,
        record! {
            a_minus_b => primitive!(2, a_minus_b),
        },
    )
}
