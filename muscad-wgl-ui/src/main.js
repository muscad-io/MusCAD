import { store, render, init } from './init'

(async () => {
  try {
    await import('../style/main.scss')
    await import('./index')

    init()
    render()
  } catch(e) {
    console.log(e)
  }
})()
