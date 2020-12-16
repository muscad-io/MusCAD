import { CodeJar } from 'codejar'
import { withLineNumbers } from 'codejar/linenumbers'

export function init(el) {
  const highlight = editor => {
    console.log(editor)
  }

  const jar = CodeJar(
    el,
    withLineNumbers(highlight),
    { indentOn: /[(\[{]$/ }
  )

  return jar
}

export default {
  init,
}
