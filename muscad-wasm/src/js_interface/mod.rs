use super::*;
use muscad_gluon::vm::Model;

#[allow(unused)]
use gluon::{
    vm::{
        api::{generic::S, IO},
        primitive, record, ExternModule, Result,
    },
    Thread,
};

fn js_log(s: String) -> IO<()> {
    log(&s);

    IO::Value(())
}

fn js_draw_model(m: &Model<S>) -> IO<()> {
    let json_str = m.with_inner(|i| serde_json::to_string(&i.0).unwrap());

    draw_model(&json_str);

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
