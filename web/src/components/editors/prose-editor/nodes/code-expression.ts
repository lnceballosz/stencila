import { Attrs, Node, NodeSpec, ParseRule } from 'prosemirror-model'
import { EditorView } from 'prosemirror-view'
import StencilaCodeExpression from '../../../nodes/code-expression'
import {
  codeExecutableAttrs,
  StencilaCodeExecutableView,
} from './code-executable'

export function codeExpression(): NodeSpec {
  return {
    group: 'InlineContent',
    inline: true,
    attrs: { ...codeExecutableAttrs, output: { default: '' } },
    parseDOM,
    toDOM,
  }
}

export class StencilaCodeExpressionView extends StencilaCodeExecutableView<StencilaCodeExpression> {
  constructor(node: Node, view: EditorView, getPos: () => number) {
    super(node, view, getPos, getAttrs, toDOM)
  }
}

const parseDOM: ParseRule[] = [
  {
    tag: 'stencila-code-expression',
    getAttrs,
  },
]

function getAttrs(node: StencilaCodeExpression): Attrs {
  return {
    id: node.id,
    programmingLanguage: node.getAttribute('programming-language'),
    guessLanguage: node.getAttribute('guess-language'),
    text: node.querySelector('[slot=text]')?.innerHTML,
    errors: node.querySelector('[slot=errors]')?.innerHTML,
    output: node.querySelector('[slot=output]')?.innerHTML,
  }
}

function toDOM(node: Node) {
  const dom = document.createElement('stencila-code-expression')
  dom.draggable = true
  dom.id = node.attrs.id
  dom.setAttribute('programming-language', node.attrs.programmingLanguage)
  dom.setAttribute('guess-language', node.attrs.guessLanguage)

  const text = document.createElement('code')
  text.slot = 'text'
  text.innerHTML = node.attrs.text
  text.contentEditable = 'false'
  dom.appendChild(text)

  const errors = document.createElement('span')
  errors.slot = 'errors'
  errors.innerHTML = node.attrs.errors
  errors.contentEditable = 'false'
  dom.appendChild(errors)

  const output = document.createElement('span')
  output.slot = 'output'
  output.innerHTML = node.attrs.output
  output.contentEditable = 'false'
  dom.appendChild(output)

  return { dom }
}
