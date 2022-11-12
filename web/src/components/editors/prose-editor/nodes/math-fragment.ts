import { Attrs, Node, NodeSpec, ParseRule } from 'prosemirror-model'
import { EditorView } from 'prosemirror-view'
import StencilaMathFragment from '../../../nodes/math-fragment'
import { StencilaMathView, mathAttrs } from './math'

export function mathFragment(): NodeSpec {
  return {
    group: 'InlineContent',
    inline: true,
    attrs: mathAttrs,
    parseDOM,
    toDOM,
  }
}

export class StencilaMathFragmentView extends StencilaMathView<StencilaMathFragment> {
  constructor(node: Node, view: EditorView, getPos: () => number) {
    super(node, view, getPos, getAttrs, toDOM)
  }
}

const parseDOM: ParseRule[] = [
  {
    tag: 'stencila-math-fragment',
    getAttrs,
  },
]

function getAttrs(node: StencilaMathFragment): Attrs {
  return {
    id: node.id,
    mathLanguage: node.getAttribute('math-language'),
    text: node.querySelector('[slot=text]')?.innerHTML,
    errors: node.querySelector('[slot=errors]')?.innerHTML,
    mathml: node.querySelector('[slot=mathml]')?.innerHTML,
  }
}

function toDOM(node: Node) {
  const dom = document.createElement('stencila-math-fragment')
  dom.contentEditable = 'false'
  dom.draggable = true
  dom.id = node.attrs.id
  dom.setAttribute('math-language', node.attrs.mathLanguage)

  const text = document.createElement('code')
  text.slot = 'text'
  text.innerText = node.attrs.text
  dom.appendChild(text)

  const errors = document.createElement('span')
  errors.slot = 'errors'
  errors.innerHTML = node.attrs.errors
  dom.appendChild(errors)

  const mathml = document.createElement('span')
  mathml.slot = 'mathml'
  mathml.innerHTML = node.attrs.mathml
  dom.appendChild(mathml)

  return { dom }
}
