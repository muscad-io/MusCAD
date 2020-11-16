use super::*;

pub fn create_vm() -> RootedThread {
    use gluon::ThreadExt;

    let vm = gluon::new_vm();

    gluon::import::add_extern_module(&vm, "intf", intf::load);

    add_extern_module(&vm, "muscad.point3d.prim", point3d::load);
    vm.load_script("muscad.point3d", include_str!("point3d/mod.glu"))
        .unwrap_or_else(|e| eprintln!("{}", e));

    add_extern_module(&vm, "muscad.model.prim", model::load);
    vm.load_script("muscad.model", include_str!("model/mod.glu"))
        .unwrap_or_else(|e| eprintln!("{}", e));

    add_extern_module(&vm, "muscad.boolean", boolean::load);

    vm
}
