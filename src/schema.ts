/**
 * Check `schema/*.schema.yaml` files and generate `built/*.schema.json`
 * from them.
 */

import fs from 'fs-extra'
import globby from 'globby'
import yaml from 'js-yaml'
import path from 'path'
import cloneDeep from 'lodash.clonedeep'
import Schema from './schema.d'
import log from './log'
import Ajv from 'ajv'
import betterAjvErrors from 'better-ajv-errors'

const SCHEMA_SOURCE_DIR = path.join(__dirname, '..', 'schema')
const SCHEMA_DEST_DIR = path.join(__dirname, '..', 'built')

// Create a validation function for JSON Schema for use in `checkSchema`
const ajv = new Ajv({ jsonPointers: true })
// eslint-disable-next-line @typescript-eslint/no-var-requires
const metaSchema = require('ajv/lib/refs/json-schema-draft-07.json')
const validateSchema = ajv.compile(metaSchema)

/**
 * Generate `built/*.schema.json` files from `schema/*.schema.yaml` files.
 */
export const build = async (): Promise<void> => {
  // Asynchronously read all the schema definition YAML files into a map of objects
  const files = await globby('*.schema.yaml', { cwd: SCHEMA_SOURCE_DIR })
  const schemas = new Map<string, Schema>(
    await Promise.all(
      files.map(
        async (file: string): Promise<[string, Schema]> => {
          const schema = yaml.safeLoad(
            await fs.readFile(path.join(SCHEMA_SOURCE_DIR, file), 'utf-8')
          )
          const title = schema.title
          if (title === undefined)
            throw new Error(`Schema title is required in source file: ${file}`)
          return [title, { ...schema, file }]
        }
      )
    )
  )
  const schemata = Array.from(schemas.values())

  // Check each schema is valid
  const fails = schemata
    .map(schema => checkSchema(schemas, schema))
    .reduce((fails, ok) => (!ok ? fails + 1 : fails), 0)
  if (fails > 0) {
    log.error(`Errors in ${fails} schemas, please see messages above`)
    // Exit with code 1 so that this fails on CI or elsewhere
    process.exit(1)
  }

  // Process each of the schemas
  schemata.forEach(schema => processSchema(schemas, schema))

  // Write to destination
  await fs.ensureDir('built')
  await Promise.all(
    Array.from(schemas.entries()).map(async ([title, schema]) => {
      const destPath = path.join(SCHEMA_DEST_DIR, title + '.schema.json')
      await fs.writeJSON(destPath, schema, { spaces: 2 })
    })
  )
}

/**
 * Run `build()` when this file is run as a Node script
 */
// eslint-disable-next-line @typescript-eslint/no-floating-promises
if (module.parent === null) build()

/**
 * Check that a schema is valid, including that,
 *
 * - is valid JSON Schema v7
 * - all type schemas (those with `properties`) have a `@id` and `description`
 * - all property schemas (those that define a property) have a `@id` and `description`
 * - that other schemas that are referred to in `extends` or `$ref` exist
 *
 * @param schemas A map of all the schemas
 * @param schema The schema being checked
 */
const checkSchema = (schemas: Map<string, Schema>, schema: Schema): boolean => {
  let valid = true
  const { title, extends: extends_, description, properties } = schema
  log.debug(`Checking type schema "${title}".`)

  const error = (message: string): void => {
    log.error(message)
    valid = false
  }

  // Is a valid schema?
  if (validateSchema(schema) !== true) {
    const message = (betterAjvErrors(
      metaSchema,
      schema,
      validateSchema.errors,
      {
        format: 'cli',
        indent: 2
      }
    ) as unknown) as string
    error(`${title} is not a valid JSON Schema:\n${message}`)
  }

  const maxDescriptionLength = 120

  // All schemas should have a description
  if (description === undefined) error(`${title} is missing description`)
  else if (description.length > maxDescriptionLength)
    error(`${title}.description is too long`)

  // Type schemas have necessary properties and extends is valid
  if (properties !== undefined) {
    if (schema['@id'] === undefined) error(`${title} is missing @id`)

    if (extends_ !== undefined) {
      if (!schemas.has(extends_))
        error(`${title}.extends refers to unknown type "${extends_}`)
    }

    // Property schemas have necessary properties
    for (const [name, property] of Object.entries(properties)) {
      if (property['@id'] === undefined)
        error(`${title}.${name} is missing @id`)

      if (property.description === undefined)
        error(`${title}.${name} is missing description`)
      else if (property.description.length > maxDescriptionLength)
        error(`${title}.${name}.description is too long`)
    }
  }

  // Check $refs are valid
  const walk = (node: Schema): void => {
    if (typeof node !== 'object') return
    for (const [key, child] of Object.entries(node)) {
      if (
        key === '$ref' &&
        typeof child === 'string' &&
        !schemas.has(child)
      ) {
        error(`${title} has a $ref to unknown type "${child}"`)
      }
      walk(child)
    }
  }
  walk(schema)

  return valid
}

/**
 * Process a schema object to implement inheritance and
 * add add derived properties.
 */
const processSchema = (schemas: Map<string, Schema>, schema: Schema): void => {
  const { $schema, $id, title, file, source, children, descendants } = schema
  log.debug(`Processing type schema "${title}".`)

  // If it's already got a children and descendants, then it's been processed.
  if (children !== undefined && descendants !== undefined) return
  schema.children = []
  schema.descendants = []

  if (title === undefined)
    throw new Error(`Schema title is required in source file: ${file}`)

  if ($schema === undefined)
    schema.$schema = `http://json-schema.org/draft-07/schema#`

  if ($id === undefined)
    schema.$id = `https://stencila.github.com/schema/${title}.schema.json`

  if (source === undefined)
    schema.source = `https://github.com/stencila/schema/blob/master/schema/${file}`

  try {
    const parent = parentSchema(schemas, schema)
    let parentProperties: { [key: string]: Schema } = {}
    let parentRequired: string[] = []
    if (parent !== null) {
      // Ensure that the parent schema has been processed (to collect properties)
      processSchema(schemas, parent)
      if (parent.properties !== undefined) parentProperties = parent.properties
      if (parent.required !== undefined) parentRequired = parent.required
    }

    // Process properties
    if (schema.properties !== undefined) {
      schema.type = 'object'

      const propertyAliases: { [key: string]: string } = {}
      for (const [name, property] of Object.entries(schema.properties)) {
        schema.properties[name].from = title
        // Registered declared aliases
        if (property.aliases !== undefined) {
          for (const alias of property.aliases) propertyAliases[alias] = name
        }
        // Add aliases for array properties (if not yet registered)
        if (property.type === 'array' && name.endsWith('s')) {
          const alias = name.slice(0, -1)
          if (property.aliases === undefined) property.aliases = []
          if (!property.aliases.includes(alias)) property.aliases.push(alias)
          propertyAliases[alias] = name
        }
        // Is this an override of a property schema in parent?
        if (name in parentProperties) property.override = true
      }

      if (Object.keys(propertyAliases).length > 0)
        schema.propertyAliases = propertyAliases

      if (schema.additionalProperties === undefined)
        schema.additionalProperties = false
    }

    // Apply `extends` keyword
    if (parent !== null) {
      // Extend `properties`
      schema.properties = {
        ...cloneDeep(parentProperties),
        ...(schema.properties !== undefined ? schema.properties : {})
      }

      // Flag inherited, but newly required properties, as overrides
      for (const [name, property] of Object.entries(schema.properties)) {
        if (
          property.from !== title &&
          schema.required !== undefined &&
          schema.required.includes(name)
        )
          property.override = true
      }

      // Having done that, now we can extend `required`
      schema.required = [
        ...parentRequired,
        ...(schema.required !== undefined ? schema.required : [])
      ]

      // Initialize the `type` property
      if (schema.properties.type !== undefined) {
        schema.properties.type = {
          ...schema.properties.type,
          enum: [title],
          default: title
        }
      }

      // Add to parent's children
      parent.children =
        parent.children === undefined
          ? [title]
          : [...parent.children, title].sort()

      // Add to all ancestors' descendants and type enum
      let ancestor: Schema | null = parent
      while (ancestor !== null) {
        ancestor.descendants =
          ancestor.descendants === undefined
            ? [title]
            : [...ancestor.descendants, title].sort()
        if (
          ancestor.title !== undefined &&
          ancestor.properties !== undefined &&
          ancestor.properties.type !== undefined &&
          ancestor.properties.type.enum !== undefined
        ) {
          ancestor.properties.type.enum = [
            ancestor.title,
            ...ancestor.descendants
          ]
        }
        ancestor = parentSchema(schemas, ancestor)
      }
    }

    // Replace any `$ref`s to YAML with a ref to the JSON generated in this function
    const walk = (node: Schema): void => {
      if (typeof node !== 'object') return
      for (const [key, child] of Object.entries(node)) {
        if (
          key === '$ref' &&
          typeof child === 'string' &&
          !child.endsWith('.schema.json')
        )
          node[key] = child + '.schema.json'
        walk(child)
      }
    }
    walk(schema)
  } catch (error) {
    throw new Error(
      `Error when processing "${schema.source}": "${error.stack}"`
    )
  }
}

/**
 * Get the parent schema, if any, of a schema
 */
const parentSchema = (
  schemas: Map<string, Schema>,
  schema: Schema
): Schema | null => {
  if (schema.extends === undefined) return null

  const parent = schemas.get(schema.extends)
  if (parent === undefined)
    throw new Error(`Unknown schema used in "extends": "${schema.extends}"`)

  return parent
}
