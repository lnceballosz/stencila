/**
 * Generate Typescript language bindings.
 */

import fs from 'fs-extra'
import { camelCase } from 'lodash'
import path from 'path'
import prettier from 'prettier'
import {
  autogeneratedHeader,
  filterEnumSchemas,
  filterInterfaceSchemas,
  filterUnionSchemas,
  getSchemaProperties,
  readSchemas,
} from '../util/helpers'
import { JsonSchema } from '../JsonSchema'

/**
 * Runs Prettier to beautify code contents based on the project settings
 */
const prettify = async (contents: string): Promise<string> => {
  const config = await prettier
    .resolveConfigFile()
    .then((path) => (path !== null ? prettier.resolveConfig(path) : undefined))

  return prettier.format(
    contents,
    config !== null ? { ...config, parser: 'typescript' } : undefined
  )
}

/**
 * Generate `../types.ts` from schemas.
 */
export const generateTypeDefinitions = async (): Promise<string> => {
  const schemas = await readSchemas()

  const code = `/* eslint-disable */

${autogeneratedHeader('build:ts', path.basename(__filename), '//')}

type Null = null
type Boolean = boolean
type Integer = number
type Number = number
type String = string
type Object = Record<string, unknown>
type Primitives = undefined | null | boolean | number | string | Array<unknown> | Object

// Remove properties from an Object if their value is undefined
const compact = <O extends object>(o: O): O =>
  Object.entries(o).reduce(
    (compactedO: O, [k, v]) =>
      v === undefined ? compactedO : { ...compactedO, [k]: v },
    {} as O
  )

${typesInterface(schemas)}

${filterInterfaceSchemas(schemas).map(interfaceGenerator).join('')}

${filterUnionSchemas(schemas).map(unionGenerator).join('')}

${filterEnumSchemas(schemas).map(enumGenerator).join('')}

${await generateTypeMaps()}
`

  const file = path.join(__dirname, '..', 'types.ts')
  await fs.writeFile(file, await prettify(code))

  return file
}

/**
 * Generate a `interface Types`, that maps all types
 * and can be used to get a type from its name at compile time.
 */
export const typesInterface = (schemas: JsonSchema[]): string => {
  return `export interface Types {\n${schemas
    .map(({ title }) =>
      title !== undefined ? `  ${title}: ${titleToType(title)}` : ''
    )
    .join('\n')}\n}`
}

/**
 * Convert the `title` of a JSON Schema to the name of a Typescript
 * type, including the interfaces and unions generated below.
 */
export const titleToType = (title: string): string => {
  switch (title) {
    case 'Array':
      return 'Array<unknown>'
    default:
      return title
  }
}

/**
 * Generate a `interface` and a factory function for each type.
 */
export const interfaceGenerator = (schema: JsonSchema): string => {
  const {
    title = 'Undefined',
    extends: parent,
    properties,
    description,
  } = schema
  const { own, required } = getSchemaProperties(schema)

  const type =
    properties !== undefined
      ? properties.type !== undefined
        ? properties.type.enum !== undefined
          ? properties.type.enum.map((type) => `'${type}'`).join(' | ')
          : ''
        : ''
      : ''

  let code = ''

  // Interface
  code += docComment(description)
  code += `export type ${title} = ${
    parent !== undefined ? `${parent} &` : ''
  } {\n`
  code += `  type: ${type}\n`
  code += own
    .map(
      ({ name, schema, optional }) =>
        `  ${name}${optional ? `?` : ''}: ${schemaToType(schema)}`
    )
    .join('\n')
  code += '\n}\n\n'

  // Factory function
  code += docComment(`Create a \`${title}\` node`, [
    `@param props Object containing ${title} schema properties as key/value pairs`,
    `@returns {${title}} ${title} schema node`,
  ])
  code += `export const ${funcName(title)} = (\n`
  const propsType = `Omit<${title}, 'type'>`
  const propsDefault = required.length <= 0 ? ' = {}' : ''
  code += `  props: ${propsType}${propsDefault}\n`
  code += `): ${title} => ({\n`
  code += `  ...compact(props),\n`
  code += `  type: '${title}'\n`
  code += '})\n\n'

  return code
}

/**
 * Generate a TypeScript "union" type.
 */
export const unionGenerator = (schema: JsonSchema): string => {
  const { title = '', description } = schema
  return (
    docComment(description) +
    `export type ${title} = ${schemaToType(schema)}\n\n`
  )
}

/**
 * Generate a Typescript `enum`.
 */
export const enumGenerator = (schema: JsonSchema): string => {
  const { title = '', description, anyOf = [] } = schema
  return (
    docComment(description) +
    `export enum ${title} {${anyOf
      .map(
        ({ description = '', const: const_ = '' }) => `
  /**
   * ${description}
   */
  ${const_ as string} = '${const_ as string}',
`
      )
      .join('')}
}\n\n`
  )
}

/**
 * Generate factory function name
 */
const funcName = (name: string): string => {
  const func = `${name.substring(0, 1).toLowerCase() + name.substring(1)}`
  const reserved: { [key: string]: string } = {
    delete: 'del',
    function: 'function_',
  }
  if (reserved[func] !== undefined) return reserved[func]
  else return func
}

/**
 * Generate a JSDoc style comment
 */
const docComment = (description?: string, tags: string[] = []): string => {
  description = description !== undefined ? description : ''
  return (
    '/**\n' +
    ' * ' +
    description.trim().replace('\n', '\n * ') +
    '\n' +
    tags.map((tag) => ' * ' + tag.trim().replace('\n', ' ') + '\n').join('') +
    ' */\n'
  )
}

/**
 * Convert a JSON Schema definition to a Typescript type
 */
const schemaToType = (schema: JsonSchema): string => {
  const { type, anyOf, allOf, $ref } = schema

  if ($ref !== undefined) return $refToType($ref)
  if (anyOf !== undefined) return anyOfToType(anyOf)
  if (allOf !== undefined) return allOfToType(allOf)
  if (schema.enum !== undefined) return enumToType(schema.enum)

  if (type === 'null') return 'Null'
  if (type === 'boolean') return 'Boolean'
  if (type === 'integer') return 'Integer'
  if (type === 'number') return 'Number'
  if (type === 'string') return 'String'
  if (type === 'array') return arrayToType(schema)
  if (type === 'object') return 'Object'

  throw new Error(`Unhandled schema: ${JSON.stringify(schema)}`)
}

/**
 * Convert a JSON Schema `$ref` (reference) to a Typescript type
 *
 * Assume that any `$ref`s refer to a type defined in the file.
 */
const $refToType = ($ref: string): string => {
  return titleToType($ref.replace('.schema.json', ''))
}

/**
 * Convert a JSON Schema with the `anyOf` property to a Typescript `Union` type.
 */
const anyOfToType = (anyOf: JsonSchema[]): string => {
  const types = anyOf
    .map((schema) => schemaToType(schema))
    .reduce(
      (prev: string[], curr) => (prev.includes(curr) ? prev : [...prev, curr]),
      []
    )
  if (types.length === 0) return ''
  if (types.length === 1) return types[0]
  return types.join(' | ')
}

/**
 * Convert a JSON Schema with the `allOf` property to a Typescript type.
 *
 * If the `allOf` is singular then just use that (this usually arises
 * because the `allOf` is used for a property with a `$ref`). Otherwise,
 * use the last schema (this is usually because one or more codecs can be
 * used on a property and the last schema is the final, expected, type of
 * the property).
 */
const allOfToType = (allOf: JsonSchema[]): string => {
  if (allOf.length === 1) return schemaToType(allOf[0])
  else return schemaToType(allOf[allOf.length - 1])
}

/**
 * Convert a JSON Schema with the `array` property to a Typescript `Array` type.
 *
 * Uses the more explicity `Array<>` syntax over the shorter`[]` syntax
 * because the latter necessitates the use of, sometime superfluous, parentheses.
 */
const arrayToType = (schema: JsonSchema): string => {
  const items = Array.isArray(schema.items)
    ? anyOfToType(schema.items)
    : schema.items !== undefined
    ? schemaToType(schema.items)
    : 'unknown'
  return `Array<${items}>`
}

/**
 * Convert a JSON Schema with the `enum` property to Typescript "or values".
 */
export const enumToType = (enu: (string | number)[]): string => {
  return enu
    .map((schema) => {
      return JSON.stringify(schema)
    })
    .join(' | ')
}

/**
 * Generate type maps for union types to be used for TypeScript type guards
 * and runtime validation.
 */
export const generateTypeMaps = async (): Promise<string> => {
  const unions = await readSchemas([
    path.join(__dirname, '..', '..', 'public', '*Types.schema.json'),
    path.join(__dirname, '..', '..', 'public', 'BlockContent.schema.json'),
    path.join(__dirname, '..', '..', 'public', 'InlineContent.schema.json'),
  ])

  // `BlockContent` & `InlineContent` schema don't have a `*Types.schema.json` file
  // This standardizes the type map names so that they all end with `Types`.
  const schemaClass = (title: string): string =>
    title?.endsWith('Types') ? title : `${title}Types`

  const unionTypes = unions
    .map((schema) => {
      const { title = '' } = schema
      return `  ${title}: ${title}`
    })
    .join('\n')

  const unionMaps = unions
    .map((schema) => {
      const { title = '' } = schema
      return `  ${title}: ${camelCase(schemaClass(title))}`
    })
    .join(',\n')

  const typeMaps = unions
    .map((schema) => {
      const { title = '' } = schema
      return `
    export const ${camelCase(
      schemaClass(title)
    )}: TypeMap<Exclude<${title}, Primitives>> = {
      ${
        schema.anyOf
          ?.reduce((typeMap: string[], type) => {
            const name = type.$ref?.replace('.schema.json', '')
            return name !== undefined &&
              ![
                'Null',
                'Boolean',
                'Integer',
                'Number',
                'String',
                'Array',
                'Object',
              ].includes(name)
              ? [...typeMap, `${name}: '${name}',`]
              : typeMap
          }, [])
          .join('\n') ?? ''
      }
      }
    `
    })
    .join()

  return `
  export type TypeMap<T extends Entity = Entity> = { [key in T['type']]: key }

  ${typeMaps}

  export interface Unions {
${unionTypes}
  }

  export const unions = {
${unionMaps}
  }
`
}

/** Generate Type Definitions and Type Maps files */
export const build = async (): Promise<void> => {
  await generateTypeDefinitions()
}

/**
 * Run `build()` when this file is run as a Node script
 */
// eslint-disable-next-line @typescript-eslint/no-floating-promises
if (module.parent === null) build()
