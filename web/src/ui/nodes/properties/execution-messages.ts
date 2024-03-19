import { MessageLevel, NodeType } from '@stencila/types'
import { apply } from '@twind/core'
import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators'

import { withTwind } from '../../../twind'
import { executionMessageUI, nodeUi } from '../icons-and-colours'

import './generic/collapsible'

/**
 * A component for displaying the `executionMessages` property of executable nodes
 */
@customElement('stencila-ui-node-execution-messages')
@withTwind()
export class UINodeExecutionMessages extends LitElement {
  @property()
  type: NodeType

  @property({ type: Number, attribute: 'message-count' })
  messageCount: number = 0

  @property({ type: Number, attribute: 'warning-count' })
  warningCount: number = 0

  @property({ type: Number, attribute: 'error-count' })
  errorCount: number = 0

  override render() {
    return html`
      <stencila-ui-node-collapsible-property
        type=${this.type}
        icon-name="terminal"
        icon-library="default"
        wrapper-css=${this.messageCount > 0 ? '' : 'hidden'}
        ?collapsed=${this.warningCount + this.errorCount == 0}
      >
        <div slot="title" class="flex justify-between mr-2">
          <span>Messages</span>
          <div class="flex">
            ${this.warningCount > 0
              ? this.renderLozenge('Warning', this.warningCount)
              : ''}
            ${this.errorCount > 0
              ? this.renderLozenge('Error', this.errorCount)
              : ''}
          </div>
        </div>
        <div slot="content" class="flex flex-col gap-y-3">
          <slot></slot>
        </div>
      </stencila-ui-node-collapsible-property>
    `
  }

  renderLozenge = (level: MessageLevel, count: number) => {
    const { icon, colour } = executionMessageUI(level)
    const { colour: nodeColour } = nodeUi(this.type)

    const styles = apply([
      'flex items-center',
      `bg-[${nodeColour}]`,
      `text-sm text-${colour}`,
      'px-1.5 ml-1',
      'rounded-full',
    ])

    return html`
      <div class=${styles}>
        <sl-icon name=${icon} library="default"></sl-icon>
        <span class="ml-1">${count}</span>
      </div>
    `
  }
}
