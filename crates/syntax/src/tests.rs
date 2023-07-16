#[cfg(test)]
mod tests {

    use crate::parser::parse;
    use expect_test::{expect, Expect};

    fn check(input: &str, expected_tree: Expect) {
        let parse = parse(input);
        expected_tree.assert_eq(&parse.debug_tree());
    }

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"SOURCE@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
SOURCE@0..3
  INT_NUMBER@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1 + 2",
            expect![[r#"
SOURCE@0..3
  BINARY_OP@0..3
    INT_NUMBER@0..1 "1"
    PLUS@1..2 "+"
    INT_NUMBER@2..3 "2""#]],
        );
    }

    // #[test]
    // fn test_what_cst() {
    //     let test_text = r#"
    //     1 * 2 + 3
    // "#;

    //     let mut parser = crate::clia_ts_parser();
    //     let tree = parser.parse(test_text, None).unwrap();
    //     let cursor = tree.walk();

    //     for node in tree_sitter_traversal::traverse(cursor, tree_sitter_traversal::Order::Pre) {
    //         dbg!(node);
    //         dbg!(node.utf8_text(test_text.as_bytes()).unwrap());
    //     }
    // }

    // #[test]
    // fn test_ungram_smoke() {
    //     let grammar = crate::clia_grammar();
    //     drop(grammar)
    // }
}
