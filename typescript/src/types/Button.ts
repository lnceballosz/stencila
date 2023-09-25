// Generated file; do not edit. See `../rust/schema-gen` crate.

import { CodeExecutable } from "./CodeExecutable.js";
import { Cord } from "./Cord.js";

/**
 * A button.
 */
export class Button extends CodeExecutable {
  type = "Button";

  /**
   * The name of the variable associated with the button.
   */
  name: string;

  /**
   * A label for the button
   */
  label?: string;

  /**
   * Whether the button is currently disabled
   */
  isDisabled?: boolean;

  constructor(code: Cord, programmingLanguage: string, name: string, options?: Partial<Button>) {
    super(code, programmingLanguage);
    if (options) Object.assign(this, options);
    this.code = code;
    this.programmingLanguage = programmingLanguage;
    this.name = name;
  }

  /**
  * Create a `Button` from an object
  */
  static from(other: Button): Button {
    return new Button(other.code!, other.programmingLanguage!, other.name!, other);
  }
}
