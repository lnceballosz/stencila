import { NodeType } from '@stencila/types'
import { apply } from '@twind/core'
import { LitElement, html } from 'lit'
import { customElement, property } from 'lit/decorators'

import '../../../buttons/chevron'
import { withTwind } from '../../../../twind'
import { nodeUi } from '../../icons-and-colours'

@customElement('stencila-ui-node-collapsible-property')
@withTwind()
export class UINodeCollapsibleProperty extends LitElement {
  @property()
  type: NodeType

  @property({ attribute: 'icon-name' })
  iconName: string

  @property({ attribute: 'icon-library' })
  iconLibrary: 'stencila' | 'default' = 'stencila'

  @property({ type: Boolean })
  collapsed: boolean = true

  @property({ attribute: 'wrapper-css' })
  wrapperCSS: string | undefined = undefined

  override render() {
    const { borderColour: headerBg } = nodeUi(this.type)

    const contentClasses = apply([
      this.collapsed ? 'max-h-0' : 'max-h-[1500px]',
      'transition-max-h duration-200',
    ])

    return html`
      <div class=${`overflow-hidden ${this.wrapperCSS ?? ''}`}>
        <div
          class=${`flex flex-row items-center px-6 py-3 cursor-pointer ${headerBg ? `bg-[${headerBg}]` : ''}`}
          @click=${() => (this.collapsed = !this.collapsed)}
        >
          ${this.iconName &&
          html`<sl-icon
            name=${this.iconName}
            library=${this.iconLibrary}
            class="text-base"
          ></sl-icon>`}

          <div class=${`grow select-none ${this.iconName && 'ml-4'}`}>
            <slot name="title"></slot>
          </div>
          <stencila-chevron-button
            .position=${this.collapsed ? 'left' : 'down'}
          ></stencila-chevron-button>
        </div>
        <div class=${contentClasses}>
          <slot name="content"></slot>
        </div>
      </div>
    `
  }
}
