/**
 * Browser bundle for user mode `Interact`
 *
 * Adds web components necessary for interacting with the document
 * but not for inspecting of modifying its execution.
 */

import { Mode, elevateMode } from '../mode'

// @ts-ignore
import('./view').then(() => elevateMode(Mode.Interact)).catch(console.error)

//export { default as StencilaParameter } from '../components/nodes/parameter'
//export { default as StencilaFilter } from '../components/nodes/filter'
//export { default as StencilaGate } from '../components/nodes/gate'
//export { default as StencilaForm } from '../components/nodes/form'
