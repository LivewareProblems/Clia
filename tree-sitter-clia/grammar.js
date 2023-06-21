const PREC = {
  ADD_OPS: 170,
  MULT_OPS: 180,
  POWER_OP: 190,
  UNARY_OPS: 200,
};

const ADD_OPS = ["+", "-"];
const MULT_OPS = ["*", "/"];
const UNARY_OPS = ["+", "-"];

const DIGITS = /[0-9]+/;
const BIN_DIGITS = /[0-1]+/;
const OCT_DIGITS = /[0-7]+/;
const HEX_DIGITS = /[0-9a-fA-F]+/;

// TODO: Actually handle unary operator so that binary operators do not get eaten without spaces
// TODO: and so that you can negate a variable
const NUMBER_DEC = seq(optional(choice(...UNARY_OPS)), sep1(DIGITS, "_"));
const NUMBER_BIN = seq(optional(choice(...UNARY_OPS)), "0b", sep1(BIN_DIGITS, "_"));
const NUMBER_OCT = seq(optional(choice(...UNARY_OPS)), "0o", sep1(OCT_DIGITS, "_"));
const NUMBER_HEX = seq(optional(choice(...UNARY_OPS)), "0x", sep1(HEX_DIGITS, "_"));

const INTEGER = choice(NUMBER_DEC, NUMBER_BIN, NUMBER_OCT, NUMBER_HEX);

const FLOAT_SCIENTIFIC_PART = seq(/[eE]/, optional(choice("-", "+")), INTEGER);
const FLOAT = seq(NUMBER_DEC, ".", NUMBER_DEC, optional(FLOAT_SCIENTIFIC_PART));

const NEWLINE = /\r?\n/;

module.exports = grammar({
  name: 'Clia',

  rules: {
    source_file: $ => repeat($._top_level),

    _top_level: $ => choice(
      $._definition,
      $._expression
    ),

    _definition: $ => choice(
      $.function_definition,
      $.module_definition
    ),

    function_definition: $ => seq(
      'def',
      $.identifier,
      $.parameter_list,
      $.function_block
    ),

    parameter_list: $ => seq(
      '(',
      // TODO: actually handle parameters, probably identifiers at first
      ')'
    ),

    function_block: $ => seq(
      'do',
      repeat($._expression),
      'end'
    ),

    _expression: $ => choice(
      $._literal,
      $.binary_op
      // TODO: handle the rest
    ),

    _literal: $ => choice(
      $.integer,
      $.atom,
      $.float
      // TODO: add more literals
    ),

    integer: ($) => token(INTEGER),

    float: ($) => token(FLOAT),

    atom: $ => seq(
      ':',
      $.identifier
      // TODO: handle full binary atoms and aliases
      // TODO: Handle proper atoms
    ),

    module_definition: $ => seq(
      'defmodule',
      $.alias,
      $.module_block
    ),

    alias: $ => /[A-Z][a-z|.|A-Z]+[a-z|A-Z]/, // TODO: make sure that this is right

    module_block: $ => seq(
      'do',
      repeat($.function_definition),
      'end'
    ),

    identifier: $ => /[a-z|_]+/, // TODO: make sure this is right

    binary_op: $ => choice(
      binaryOp($, prec.left, PREC.ADD_OPS, choice(...ADD_OPS)),
      binaryOp($, prec.left, PREC.MULT_OPS, choice(...MULT_OPS)),
      binaryOp($, prec.left, PREC.POWER_OP, "**"),
    )
  },
});

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}

function binaryOp($, assoc, precedence, operator, left = null, right = null) {
  return assoc(
    precedence,
    seq(
      field("left", left || $._expression),
      field("operator", operator),
      field("right", right || $._expression)
    )
  );
}
