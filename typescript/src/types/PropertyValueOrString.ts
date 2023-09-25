// Generated file; do not edit. See `../rust/schema-gen` crate.
            
import { PropertyValue } from "./PropertyValue.js";

/**
 * `PropertyValue` or `string`
 */
export type PropertyValueOrString =
  PropertyValue |
  string;

/**
 * Create a `PropertyValueOrString` from an object
 */
export function propertyValueOrString(other: PropertyValueOrString): PropertyValueOrString {
  if (other == null || typeof other !== "object" || Array.isArray(other) || typeof other.type === "undefined") {
    return other as PropertyValueOrString;
  }
  switch(other.type) {
    case "PropertyValue": return PropertyValue.from(other as PropertyValue);
    default: throw new Error(`Unexpected type for PropertyValueOrString: ${other.type}`);
  }
}
