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
        check("", expect![[r#"Source@0..0"#]]);
    }

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
                Source@0..3
                  Literal@0..3
                    Integer@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1 + 2",
            expect![[r#"
                Source@0..3
                  BinaryOp@0..3
                    Literal@0..1
                      Integer@0..1 "1"
                    Plus@1..2 "+"
                    Literal@2..3
                      Integer@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_comment() {
        check(
            "# hello!",
            expect![[r##"
              Source@0..8
                Comment@0..8 "# hello!""##]],
        );
    }

    #[test]
    fn parse_binary_expression_interspersed_with_comments() {
        check(
            "
              1
                + 1 # Add one
                + 10 # Add ten",
            expect![[r##"
            Source@0..24
              BinaryOp@0..15
                BinaryOp@0..3
                  Literal@0..1
                    Integer@0..1 "1"
                  Plus@1..2 "+"
                  Literal@2..3
                    Integer@2..3 "1"
                Comment@3..12 "# Add one"
                Plus@12..13 "+"
                Literal@13..15
                  Integer@13..15 "10"
              Comment@15..24 "# Add ten""##]],
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
