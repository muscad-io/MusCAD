extern crate muscad_core as muscad;
extern crate muscad_gluon;

use muscad_gluon::gluon;

use wasm_bindgen::prelude::*;

mod js_interface;
//pub mod threejs;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct VmWrapper(gluon::RootedThread);

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = draw)]
    fn draw_model(s: &str);
}

#[wasm_bindgen]
pub fn setup_vm() -> VmWrapper {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    use muscad_gluon::ThreadExt;

    let vm = muscad_gluon::vm::create_vm();

    gluon::import::add_extern_module(&vm, "intf", js_interface::load);

    vm.run_io(true);

    VmWrapper(vm)
}

#[wasm_bindgen]
pub fn run_script(vm: VmWrapper, body: String) {
    use muscad_gluon::ThreadExt;
    vm.0.load_script(&"unnamed.glu", &body)
        .unwrap_or_else(|e| log(&format!("{}", e)));
}
