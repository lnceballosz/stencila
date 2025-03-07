// Generated file; do not edit. See https://github.com/stencila/stencila/tree/main/rust/schema-gen

import { Inline } from "./Inline.js";
import { Instruction } from "./Instruction.js";
import { InstructionMessage } from "./InstructionMessage.js";
import { InstructionType } from "./InstructionType.js";
import { SuggestionInline } from "./SuggestionInline.js";

/**
 * An instruction to edit some inline content.
 */
export class InstructionInline extends Instruction {
  // @ts-expect-error 'not assignable to the same property in base type'
  type: "InstructionInline";

  /**
   * The content to which the instruction applies.
   */
  content?: Inline[];

  /**
   * Suggestions for the instruction
   */
  suggestions?: SuggestionInline[];

  constructor(instructionType: InstructionType, messages: InstructionMessage[], options?: Partial<InstructionInline>) {
    super(instructionType, messages);
    this.type = "InstructionInline";
    if (options) Object.assign(this, options);
    this.instructionType = instructionType;
    this.messages = messages;
  }
}

/**
* Create a new `InstructionInline`
*/
export function instructionInline(instructionType: InstructionType, messages: InstructionMessage[], options?: Partial<InstructionInline>): InstructionInline {
  return new InstructionInline(instructionType, messages, options);
}
