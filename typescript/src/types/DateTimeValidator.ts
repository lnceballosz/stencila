// Generated file; do not edit. See `../rust/schema-gen` crate.

import { DateTime } from "./DateTime.js";
import { Entity } from "./Entity.js";

/**
 * A validator specifying the constraints on a date-time.
 */
export class DateTimeValidator extends Entity {
  type = "DateTimeValidator";

  /**
   * The inclusive lower limit for a date-time.
   */
  minimum?: DateTime;

  /**
   * The inclusive upper limit for a date-time.
   */
  maximum?: DateTime;

  constructor(options?: Partial<DateTimeValidator>) {
    super();
    if (options) Object.assign(this, options);
    
  }

  /**
  * Create a `DateTimeValidator` from an object
  */
  static from(other: DateTimeValidator): DateTimeValidator {
    return new DateTimeValidator(other);
  }
}
