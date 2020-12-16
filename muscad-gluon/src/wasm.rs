use super::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = draw)]
    pub fn draw_model(s: &str);
}

#[wasm_bindgen]
pub struct VmWrapper(gluon::RootedThread);

#[wasm_bindgen]
pub fn setup_vm() -> VmWrapper {
    let vm = vm::create_vm();
    vm.run_io(true);

    VmWrapper(vm)
}

#[wasm_bindgen]
pub fn run_script(vm: &VmWrapper, body: String) {
    vm.0.load_script(&"unnamed.glu", &body)
        .unwrap_or_else(|e| log(&format!("{}", e)));
}

#[wasm_bindgen]
pub fn run_vm() -> () {
    let vm = vm::create_vm();
    vm.run_io(true);

    vm.load_script(
        &"unnamed.glu",
        "let io = import! std.io in io.print \"Hello world!\"",
    )
    .unwrap_or_else(|e| println!("{}", e));
}
