import * as wasm from 'muscad-wasm'

const vm = wasm.setup_vm()

window.wasm = wasm
window.vm = vm
