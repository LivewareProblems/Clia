#[cfg(test)]
mod tests;

mod syntax_kind;
mod parser;

use parser::{parse, Parse};
use syntax_kind::CliaLang;
use tree_sitter::{self, Parser};

pub type SyntaxNode = rowan::SyntaxNode<CliaLang>;
pub type SyntaxToken = rowan::SyntaxToken<CliaLang>;
pub type SyntaxElement = rowan::SyntaxElement<CliaLang>;
pub type SyntaxKind = syntax_kind::SyntaxKind;

pub fn clia_ts_parser() -> Parser {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_clia::language()).expect("Error loading Clia grammar");
    return parser;
}

pub fn clia_to_cst(input: &str) -> Parse {
    parse(input)
}
