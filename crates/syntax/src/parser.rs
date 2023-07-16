use crate::clia_ts_parser;
use crate::syntax_kind::{CliaLang, SyntaxKind, SyntaxKind::*};
use crate::SyntaxNode;

use rowan::GreenNodeBuilder;
use rowan::{GreenNode, Language};
use tree_sitter::{Node, Tree, TreeCursor};

pub struct Parse {
    green_node: GreenNode,
    #[allow(unused)]
    errors: Vec<String>,
}

pub(crate) fn parse(input: &str) -> Parse {
    let mut ts_parser = clia_ts_parser();
    let tree = ts_parser.parse(input, None).unwrap();
    let parser = Parser::new(tree, input);

    parser.parse()
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }

    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

pub(crate) struct Parser<'a> {
    ts_tree: Tree,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
    text: &'a str,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(tree: Tree, input: &'a str) -> Self {
        Self { ts_tree: tree, builder: GreenNodeBuilder::new(), errors: Vec::new(), text: input }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(CliaLang::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    pub(crate) fn parse(mut self) -> Parse {
        if SyntaxKind::from_ts_kind(self.ts_tree.root_node().kind()) == Some(SOURCE) {
            self.start_node(SOURCE);
            let tree_cl = self.ts_tree.clone();
            let mut cursor = tree_cl.walk();
            cursor.goto_first_child();
            self.traverse_and_parse(&mut cursor);
        } else {
            self.errors.push("wrong source".to_string())
        }

        Parse { green_node: self.builder.finish(), errors: self.errors }
    }

    fn traverse_and_parse(&mut self, cursor: &mut TreeCursor) -> () {
        'outer: loop {
            let node = cursor.node();
            let kind = SyntaxKind::from_ts_kind(node.kind()).unwrap_or(ERROR);
            if kind.is_token() {
                self.token_from_kind(kind, node);
                'inner: loop {
                    if !cursor.goto_next_sibling() {
                        if !cursor.goto_parent() {
                            break 'outer;
                        } else {
                            self.finish_node();
                        }
                    } else {
                        break 'inner;
                    }
                }
            } else {
                self.start_node(kind);
                if !cursor.goto_first_child() {
                    self.errors.push("Operator without token".to_string());
                    self.finish_node();
                    'inner: loop {
                        if !cursor.goto_next_sibling() {
                            if !cursor.goto_parent() {
                                break 'outer;
                            } else {
                                self.finish_node();
                            }
                        } else {
                            break 'inner;
                        }
                    }
                }
            };
        }
    }

    fn token_from_kind(&mut self, kind: SyntaxKind, node: Node) -> () {
        self.builder.token(kind.into(), node.utf8_text(self.text.as_bytes()).unwrap());
    }
}
