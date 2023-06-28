use tree_sitter::{self, Tree};
use ungrammar::Grammar;

pub fn clia_ts_tree() -> Tree {
    let code = r#"defmodule Test do
    def foo() do
      1 + 2
    end
  end
  "#;
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(tree_sitter_clia::language())
        .expect("Error loading Clia grammar");
    let tree = parser.parse(code, None).unwrap();
    return tree;
}

/// Returns a Rust grammar.
pub fn clia_grammar() -> Grammar {
    let src = include_str!("../clia.ungram");
    dbg!(src.parse()).unwrap()
}
mod tests {

    #[test]
    fn test_what_cst() {
        let tree = crate::clia_ts_tree();
        let cursor = tree.walk();

        for node in tree_sitter_traversal::traverse(cursor, tree_sitter_traversal::Order::Pre) {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_ungram_smoke() {
        let grammar = crate::clia_grammar();
        drop(grammar)
    }
}
