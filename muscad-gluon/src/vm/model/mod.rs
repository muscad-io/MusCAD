use super::*;

#[derive(Debug)]
struct ModelSend(NativeModel);

unsafe impl Send for ModelSend {}

#[derive(Debug, Trace, Userdata, VmType)]
#[gluon(vm_type = "muscad.model.prim.Model")]
#[gluon_trace(skip)]
pub struct Model<S> {
    inner: Mutex<ModelSend>,
    s: PhantomData<S>,
}

impl Model<S> {
    pub fn new(m: NativeModel) -> Self {
        Self {
            inner: Mutex::new(ModelSend(m)),
            s: PhantomData,
        }
    }

    pub fn with_inner<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut NativeModel) -> R,
    {
        f(&mut self.inner.lock().unwrap().0)
    }
}

pub fn add_face(m: &Model<S>, pts: Vec<Vec<f64>>) {
    let mut pts_vec3d = Vec::with_capacity(pts.len());

    for p in pts.iter() {
        pts_vec3d.push(muscad::Point3d::from(&[p[0], p[1], p[2]]));
    }

    m.with_inner(|inner| inner.add_face(&pts_vec3d));
}

pub fn new(_: ()) -> Model<S> {
    Model::new(NativeModel::new())
}

pub fn show(m: &Model<S>) -> String {
    m.with_inner(|m| format!("{:?}", m))
}

pub fn load(vm: &Thread) -> vm::Result<ExternModule> {
    vm.register_type::<model::Model<S>>("muscad.model.prim.Model", &["s"])?;

    ExternModule::new(
        vm,
        record! {
            type Model s => Model<S>,
            new => primitive!(1, "muscad.model.prim.new", new),
            show => primitive!(1, show),
            add_face => primitive!(2, add_face),
        },
    )
}
