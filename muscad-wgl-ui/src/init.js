import * as THREE from 'three'
import chunk from 'lodash/chunk'
import InitOrbitControl from 'three-orbit-controls'
const OrbitControls = InitOrbitControl(THREE)

import { init as editor_init } from './editor'
import events from './events'

export const store = {}

export const render = () => {
  const {renderer, scene, camera} = store
  renderer.clear();
  renderer.render(scene, camera)

  window.requestAnimationFrame(render)
}

function fill_src(el) {
  el.value = require('./sample.glu').default
}

function init_html_elements(store) {
  const el_container = document.getElementById('canvas_parent')
  const el_canvas = document.getElementById('canvas')
  const el_run = document.getElementById('run')
  const el_editor = document.getElementById('editor')
  const el_outputs = document.getElementById('outputs')
  const editor = editor_init(el_editor)

  editor.updateCode(require('./sample.glu').default)

  events.on('resize', () => {
    const w = store.get_width()
    canvas.width = w
    canvas.height = 0.5 * w

    const ctrl_height = 80
    const out_height = window.innerHeight - canvas.height - ctrl_height - 10
    el_outputs.style.height = `${out_height}px`
  })

  el_run.onclick = () => {
    run_src(editor.toString())
    //el_outputs.innerHTML = editor.toString();
  }

  store.get_width = () => el_container.getBoundingClientRect().width
  store.el_container = el_container
  store.el_canvas = el_canvas
  store.editor = editor
}


function init_scene(store) {
  const scene = new THREE.Scene()
  const ambient_light = new THREE.AmbientLight(0xe0e0e0, 1)
  const dir_light = new THREE.DirectionalLight(0xdddddd, 0.25)

  scene.add(ambient_light)
  scene.add(dir_light)
  scene.add(dir_light.target)
  events.on('camera_change', (cam, ctrl) => {
    dir_light.position.copy(cam.position)
    dir_light.target.position.copy(ctrl.target)
  })

  scene.add(new THREE.AxesHelper(10000))
  store.scene = scene
}

function init_renderer(store) {
  const renderer = new THREE.WebGL1Renderer({
    canvas: store.el_canvas,
    alpha: true,
    antialias: true
  })

  renderer.setPixelRatio(window.devicePixelRatio)
  renderer.setClearColor(0xFEFEFE, 1)

  events.on('resize', () => {
    const w = store.get_width()

    renderer.setViewport(0, 0, w, 0.5 * w)
  })

  store.renderer = renderer
}

function init_camera(store) {
  const camera = new THREE.PerspectiveCamera(50, 2, 1, 10000)
  camera.position.z = 50
  camera.lookAt(0,0,0)

  store.camera = camera
  store.ctrl_orbit = new OrbitControls(store.camera, store.el_canvas)
  store.ctrl_orbit.addEventListener('change', () => {
    events.emit('camera_change', store.camera, store.ctrl_orbit)
  })
}

function run_src(script) {
  wasm.run_script(window.vm, script)
}

function _draw_model(store, raw) {
  const json = JSON.parse(raw)

  const position = new THREE.Float32BufferAttribute(json.pts.flat(), 3)

  const face_material = new THREE.MeshPhongMaterial({
    color : 0x00cc00 ,
    side: THREE.DoubleSide,
    polygonOffset: true,
    polygonOffsetFactor: 1,
    polygonOffsetUnits: 1
  })

  const face_geometry = new THREE.BufferGeometry()
  face_geometry.setAttribute('position', position)
  face_geometry.setIndex(json['triangles'])

  face_geometry.computeFaceNormals()
  face_geometry.computeVertexNormals()
  const mesh = new THREE.Mesh(face_geometry, face_material)
  store.scene.add(mesh)

  const line_geometry = new THREE.BufferGeometry()
  line_geometry.setAttribute('position', position)
  line_geometry.setIndex(json['lines'])
  line_geometry.computeBoundingSphere();
  const line_material = new THREE.LineBasicMaterial({ color: 0x000000 })
  const lines = new THREE.LineSegments(line_geometry, line_material)
  store.scene.add(lines)

}

export function init() {
  init_html_elements(store)
  init_scene(store)
  init_camera(store)
  init_renderer(store)

  window.addEventListener('resize', () => events.emit('resize'))
  events.emit('resize')

  console.log(store)

  window.draw = {}
  window.draw.draw_model = _draw_model.bind(null, store)
}
