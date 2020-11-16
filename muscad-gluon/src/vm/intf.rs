use super::*;

use gluon::{
    vm::{
        api::{generic::S, IO},
        primitive, record, ExternModule, Result,
    },
    Thread,
};

fn js_log(s: String) -> IO<()> {
    println!("{}", s);

    IO::Value(())
}

fn js_draw_model(_m: &Model<S>) -> IO<()> {
    println!("<draw call>");

    IO::Value(())
}

pub fn load(vm: &Thread) -> Result<ExternModule> {
    ExternModule::new(
        vm,
        record! {
            log => primitive!(1, js_log),
            draw_model => primitive!(1, js_draw_model),
        },
    )
}
