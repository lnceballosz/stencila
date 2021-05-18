// Node.js bindings for ../../rust/src/projects.rs, see there for more documentation.

import { JSONSchema7 } from 'json-schema'
import { fromJSON } from './prelude'
import { subscribe } from './pubsub'
import { Project } from './types'

const addon = require('../index.node')

/**
 * Get the JSON schema for a project object
 *
 * @returns A JSON Schema v7 object describing the properties of
 *          a project object
 */
export function schema(): JSONSchema7 {
  return fromJSON<JSONSchema7>(addon.projectsSchema())
}

/**
 * List projects that are currently open
 *
 * @returns An array of projects
 */
export function list(): Project[] {
  return fromJSON<Project[]>(addon.projectsList())
}

/**
 * Open a project
 *
 * @param path Path to the project folder
 * @param subscriber A subscriber function that will receive published
 *                   events for the project
 * @return A project
 */
export function open(
  folder: string,
  subscriber?: (topic: string, event: unknown) => unknown
): Project {
  const project = fromJSON<Project>(addon.projectsOpen(folder))
  if (subscriber !== undefined) subscribe(`project:${project.path}`, subscriber)
  return project as Project
}

/**
 * Close a project
 *
 * @param Path to the project folder
 */
export function close(folder: string): void {
  addon.projectsClose(folder)
}
