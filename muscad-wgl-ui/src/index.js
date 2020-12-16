import * as wasm from 'muscad-gluon'

const vm = wasm.setup_vm()

window.wasm = wasm
window.vm = vm
