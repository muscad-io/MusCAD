#[macro_use]
extern crate gluon_codegen;
#[macro_use]
extern crate gluon_vm;

extern crate muscad_core as muscad;

//extern crate wee_alloc;

//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use gluon::ThreadExt;

pub use gluon;
pub mod vm;

pub mod wasm;

//mod wasm {
//    pub fn alert(s: &str) {
//        println!("{}", s);
//    }
//
//    pub fn log(s: &str) {
//        println!("{}", s);
//    }
//
//    pub fn draw_model(s: &str) {
//        println!("<draw_model>");
//    }
//}
