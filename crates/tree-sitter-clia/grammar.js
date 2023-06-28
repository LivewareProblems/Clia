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
const NUMBER_DEC = seq(sep1(DIGITS, "_"));
const NUMBER_BIN = seq("0b", sep1(BIN_DIGITS, "_"));
const NUMBER_OCT = seq("0o", sep1(OCT_DIGITS, "_"));
const NUMBER_HEX = seq("0x", sep1(HEX_DIGITS, "_"));

const INTEGER = choice(NUMBER_DEC, NUMBER_BIN, NUMBER_OCT, NUMBER_HEX);

const FLOAT_SCIENTIFIC_PART = seq(/[eE]/, optional(choice("-", "+")), INTEGER);
const FLOAT = seq(NUMBER_DEC, ".", NUMBER_DEC, optional(FLOAT_SCIENTIFIC_PART));

const NEWLINE = /\r?\n/;

module.exports = grammar({
  name: 'clia',

  rules: {
    source_file: $ => repeat($._top_level),

    _top_level: $ => choice(
      $._definition,
      $._expression
    ),

    _terminator: ($) =>
      // Right precedence, because we want to consume `;` after newlines if present
      prec.right(choice(seq(repeat(NEWLINE), ";"), repeat1(NEWLINE))),

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
      $.expression_block,
      $._literal,
      $.binary_op,
      $.unary_op
      // TODO: handle the rest
    ),

    expression_block: $ =>
      seq(
        "(",
        optional($._terminator),
        optional(
          choice(
            seq(
              sep1(choice($._expression), $._terminator),
              optional($._terminator)
            )
          )
        ),
        ")"
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
    ),

    unary_op: $ =>
      choice(
        unaryOp($, prec, PREC.UNARY_OPS, choice(...UNARY_OPS)),
      ),

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

function unaryOp($, assoc, precedence, operator, right = null) {
  // Expression such as `x + y` falls under the "expression vs local call"
  // conflict that we already have. By using dynamic precedence we penalize
  // unary operator, so `x + y` is interpreted as binary operator (unless
  // _before_unary_op is tokenized and forces unary operator interpretation)
  return assoc(
    precedence,
    seq(
      field("operator", operator),
      field("operand", right || $._expression)
    )
  );
}