#[allow(unused)]
use std::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
    sync::Mutex,
};

pub type VmFloat = f64;

#[allow(unused)]
use gluon::{
    import::add_extern_module,
    vm::{self, api::generic::S, ExternModule},
    RootedThread, Thread,
};

type NativeModel = muscad::Model<f64>;

pub use init::create_vm;

pub use model::*;
mod boolean;
mod init;
mod intf;
mod model;
mod point3d;
