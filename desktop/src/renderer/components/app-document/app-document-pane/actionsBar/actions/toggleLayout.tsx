import { EntityId } from '@reduxjs/toolkit'
import { FunctionalComponent, h } from '@stencil/core'
import { Document } from 'stencila'
import { state } from '../../../../../store'
import {
  isEditable,
  isEditPaneOpen,
  isPreviewable,
  isPreviewPaneOpen,
  setEditorPaneVisibility,
  setPreviewPaneVisibility,
} from '../../../../../store/documentPane/documentPaneActions'
import { selectView } from '../../../../../store/documentPane/documentPaneSelectors'
import { makeLayoutId } from '../../../../../store/documentPane/documentPaneStore'
import { PaneLayout } from '../../../../../store/documentPane/documentPaneTypes'

const cycleLayout =
  (view?: Document, layout?: PaneLayout) =>
  (paneId: EntityId, viewId: EntityId) =>
  (e: Event) => {
    e.preventDefault()
    if (!view || !layout) {
      return
    }

    const layoutId = makeLayoutId(paneId)(viewId)

    const editable = isEditable(view)
    const previewable = isPreviewable(view)
    const isPreviewOpen = isPreviewPaneOpen(layout)
    const isEditorOpen = isEditPaneOpen(layout)

    if (previewable && isPreviewOpen && editable && isEditorOpen) {
      setPreviewPaneVisibility(layoutId, false)
    } else if (previewable && !isPreviewOpen && editable && isEditorOpen) {
      setEditorPaneVisibility(layoutId, false)
      setPreviewPaneVisibility(layoutId, true)
    } else if (previewable && editable) {
      setEditorPaneVisibility(layoutId, true)
      setPreviewPaneVisibility(layoutId, true)
    } else if (previewable) {
      setPreviewPaneVisibility(layoutId, true)
    } else if (editable) {
      setEditorPaneVisibility(layoutId, true)
    }
  }

const hasNextLayout = (view: Document): boolean => {
  return isPreviewable(view) && isEditable(view) ? true : false
}

interface Props {
  paneId: EntityId
  viewId: EntityId
}

export const TogglePaneLayoutButton: FunctionalComponent<Props> = ({
  paneId,
  viewId,
}) => {
  const { view, layout } = selectView(state)(paneId)(viewId)
  const isDisabled = !(view && hasNextLayout(view)) ?? true

  return (
    <stencila-button
      iconOnly={true}
      icon="layout-5"
      color={isDisabled ? 'key' : 'neutral'}
      disabled={isDisabled}
      minimal={true}
      size="xsmall"
      tooltip="Toggle layout"
      onClick={cycleLayout(view, layout)(paneId, viewId)}
    >
      Toggle layout
    </stencila-button>
  )
}
