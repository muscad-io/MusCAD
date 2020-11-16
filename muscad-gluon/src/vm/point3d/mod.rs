use super::*;

#[derive(Trace, Getable, Pushable, VmType)]
#[gluon(vm_type = "muscad.point3d.prim.Point3d")]
#[gluon_trace(skip)]
pub struct Point3d {
    x: VmFloat,
    y: VmFloat,
    z: VmFloat,
}

impl Debug for Point3d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Point3d")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

pub fn new(_: ()) -> Point3d {
    let [x, y, z] = [0.0, 0.0, 0.0];
    Point3d { x, y, z }
}

pub fn show(p: Point3d) -> String {
    format!("{:?}", p)
}

pub fn load(vm: &Thread) -> vm::Result<ExternModule> {
    vm.register_type::<Point3d>("muscad.point3d.prim.Point3d", &[])?;

    ExternModule::new(
        vm,
        record! {
            type Point3d => Point3d,
            new => primitive!(1, "muscad.model.prim.new", new),
            show => primitive!(1, show),
        },
    )
}
