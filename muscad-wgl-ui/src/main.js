import { store, render, init } from './init'
(async () => {
  await import('../style/main.scss')
  await import('./index')

  init()
  render()
})()


