import * as schema from '@stencila/schema'
import { InlineContentArray } from './InlineContent'

export function Emphasis(props: { node: schema.Emphasis }) {
  return (
    <em itemtype="https://schema.stenci.la/Emphasis">
      <InlineContentArray nodes = {props.node.content}></InlineContentArray>
    </em>
  )
}
