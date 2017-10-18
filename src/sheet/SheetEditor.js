import { platform, DefaultDOMElement, AbstractEditor, Toolbar, EditorSession, EventEmitter } from 'substance'
import SheetLinter from './SheetLinter'
import SheetStatusBar from './SheetStatusBar'

export default class SheetEditor extends AbstractEditor {

  constructor(...args) {
    super(...args)

    this.__onResize = this.__onResize.bind(this)
    const sheet = this.getDocument()
    this.linter = new SheetLinter(sheet, this.getEditorSession())
    // _cellEditorDoc is used by cell editors (expression bar, or popover editor on enter)
    this._cellEditorDoc = sheet.newInstance()
    // Just adds one cell, used for text editing
    this._cellEditorDoc._node = this._cellEditorDoc.createElement('cell')
    this._cellEditorSession = new CellEditorSession(this._cellEditorDoc, {
      // EXPERIMENTAL: trying to setup an editor session using the same CommandManager
      // but working on a different doc
      configurator: this.context.editorSession.configurator,
      commandManager: this.context.editorSession.commandManager
    })
  }

  getChildContext() {
    let editorSession = this.props.editorSession
    return Object.assign({}, super.getChildContext(), {
      issueManager: editorSession.issueManager,
      cellEditorSession: this._cellEditorSession
    })
  }

  getInitialState() {
    return {
      showConsole: false,
      consoleContent: null
    }
  }

  didMount() {
    // always render a second time to render for the real element dimensions
    this.rerender()
    super.didMount()
    if (platform.inBrowser) {
      DefaultDOMElement.wrap(window).on('resize', this._onResize, this)
    }
    this.linter.start()
  }

  dispose() {
    super.dispose()
    if (platform.inBrowser) {
      DefaultDOMElement.wrap(window).off(this)
    }
  }

  render($$) {
    let el = $$('div').addClass('sc-sheet-editor')
    el.append(
      this._renderToolbar($$),
      this._renderContent($$),
      this._renderStatusbar($$)
    )
    return el
  }

  _renderToolbar($$) {
    const configurator = this.getConfigurator()
    return $$(Toolbar, {
      toolPanel: configurator.getToolPanel('toolbar')
    }).ref('toolbar')
  }

  _renderContent($$) {
    let el = $$('div').addClass('se-body')
    el.append(
      this._renderSheet($$)
    )
    el.append(
      this._renderConsole($$)
    )
    return el
  }

  _renderSheet($$) {
    const sheet = this.getDocument()
    const linter = this.linter
    // only rendering the sheet when mounted
    // so that we have real width and height
    if (this.isMounted()) {
      const SheetComponent = this.getComponent('sheet')
      return $$(SheetComponent, {
        sheet, linter
      }).ref('sheet')
    } else {
      return $$('div')
    }
  }

  _renderConsole($$) {
    let el = $$('div').addClass('se-console')
    if (this.state.showConsole) {
      let ConsoleContent = this.getComponent(this.state.consoleContent)
      el.append(
        $$(ConsoleContent, { editor: this })
      )
    }
    return el
  }

  _renderStatusbar($$) {
    return $$(SheetStatusBar, {}).ref('sheet-statusbar')
  }

  getLinter() {
    return this.linter
  }

  getIssues() {
    let editorSession = this.props.editorSession
    let issueManager = editorSession.issueManager
    return issueManager.getIssues('linter')
  }

  getWidth() {
    if (this.el) {
      return this.el.getWidth()
    } else {
      return 1000
    }
  }

  getHeight() {
    if (this.el) {
      return this.el.getHeight()
    } else {
      return 750
    }
  }

  toggleConsole(consoleContent) {
    if (this.state.showConsole && this.state.consoleContent === consoleContent) {
      this.setState({
        showConsole: false
      })
    } else {
      this.setState({
        showConsole: true,
        consoleContent
      })
    }
  }

  _onResize() {
    if (platform.inBrowser) {
      if (!this._rafId) {
        this._rafId = window.requestAnimationFrame(this.__onResize)
      }
    }
  }

  __onResize() {
    this._rafId = null
    this.refs.sheet.resize()
  }

}

class CellEditorSession extends EditorSession {

  /*
    Triggered when a cell editor is focused
  */
  startEditing() {
    if (!this.isEditing) {
      this.isEditing = true
      this.emit('editing:started')
    }
  }

  /*
    Triggered when cell editing is confirmed (e.g. enter is pressed in the cell editor)
  */
  confirmEditing(silent) {
    if (this.isEditing) {
      this.isEditing = false
      if (!silent) this.emit('editing:confirmed')
    }
  }

  /*
    Triggered when cell editing is cancelled (e.g. escape is pressed in the cell editor)
  */
  cancelEditing() {
    if (this.isEditing) {
      this.isEditing = false
      this.emit('editing:cancelled')
    }
  }

  /*
    Get the current value of the cell editor
  */
  getValue() {
    let node = this.getDocument()._node
    return node.getText()
  }

}
