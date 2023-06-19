module.exports = grammar({
  name: 'Clia',

  rules: {
    source_file: $ => repeat($._definition),

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
      $.literal
      // TODO: handle the rest
    ),

    literal: $ => choice(
      $.integer,
      $.atom,
      $.float
      // TODO: add more literals
    ),

    integer: $ => sep1(/\d+/, "_"),

    float: $ => seq(
      $.integer,
      '.',
      $.integer,
      optional($._float_scientific_part)
    ),

    _float_scientific_part: $ => seq(
      /[eE]/,
      optional(choice("-", "+")),
      $.integer
    ),

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

    identifier: $ => /[a-z|_]+/ // TODO: make sure this is right
  },
});

function sep1(rule, separator) {
  return seq(rule, repeat(seq(separator, rule)));
}