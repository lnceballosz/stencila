import { DomOperationReplace, Slot } from '@stencila/stencila'
import { ElementId } from '../types'
import {
  assert,
  assertElement,
  assertNumber,
  assertString,
  createFragment,
  isElement,
  isString,
  panic,
  resolveParent,
  resolveSlot,
  toGraphemes,
} from './utils'

/**
 * Apply a replace operation
 */
export function applyReplace(
  op: DomOperationReplace,
  target?: ElementId
): void {
  const { address, items, html } = op
  const [parent, slot] = resolveParent(address, target)

  if (isElement(parent)) {
    if (isString(slot)) applyReplaceOption(parent, slot, items, html)
    else applyReplaceVec(parent, slot, items, html)
  } else applyReplaceString(parent, slot, items, html)
}

/**
 * Apply a replace operation to an element representing an `Option`
 */
export function applyReplaceOption(
  node: Element,
  slot: Slot,
  items: number,
  html: string
): void {
  assertString(slot)
  assert(
    items === 1,
    `Unexpected replace items ${items} for option slot '${slot}'`
  )

  const target = resolveSlot(node, slot)
  assertElement(target)
  target.outerHTML = html
}

/**
 * Apply a replace operation to an element representing a `Vec`
 */
export function applyReplaceVec(
  node: Element,
  slot: Slot,
  items: number,
  html: string
): void {
  assertNumber(slot)

  const fragment = createFragment(html)
  const children = node.childNodes
  if (children.length === 0) {
    node.appendChild(fragment)
  } else {
    const child = children[slot]
    if (child === undefined) {
      throw panic(
        `Unexpected replace slot '${slot}' for element with ${children.length} children`
      )
    }
    node.insertBefore(fragment, child)

    let removed = 0
    while (removed < items) {
      children[slot + 1]?.remove()
      removed += 1
    }
  }
}

/**
 * Apply a replace operation to a text node representing a `String`
 */
export function applyReplaceString(
  node: Text,
  slot: Slot,
  items: number,
  value: string
): void {
  assertNumber(slot)

  const graphemes = toGraphemes(node.textContent ?? '')
  assert(
    slot >= 0 && slot <= graphemes.length,
    `Unexpected replace slot '${slot}' for text node of length ${graphemes.length}`
  )
  assert(
    items > 0 && slot + items <= graphemes.length,
    `Unexpected replace items ${items} for text node of length ${graphemes.length}`
  )
  node.textContent =
    graphemes.slice(0, slot).join('') +
    value +
    graphemes.slice(slot + items).join('')
}
