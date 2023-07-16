use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::SOURCE {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn exprs(&self) -> impl Iterator<Item = Expr> {
        self.0.children().filter_map(Expr::cast)
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Integer(Integer),
}
impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::BINARY_OP => Self::BinaryExpr(BinaryExpr(node)),
            SyntaxKind::INT_NUMBER => Self::Integer(Integer(node)),
            _ => return None,
        };

        Some(result)
    }
}
#[derive(Debug)]
pub struct Integer(SyntaxNode);

impl Integer {
    pub fn parse(&self) -> u64 {
        self.0.first_token().unwrap().text().parse().unwrap()
    }
}
#[derive(Debug)]
pub struct BinaryExpr(SyntaxNode);

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.children().filter_map(Expr::cast).nth(1)
    }

    pub fn op(&self) -> Option<SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(SyntaxElement::into_token)
            .find(|token| matches!(token.kind(), SyntaxKind::PLUS,))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syntax::clia_to_cst;

    #[test]
    fn it_works() {
        let input = "1 + 2";
        let parse = clia_to_cst(&input);
        println!("{}", parse.debug_tree());

        let root = Root::cast(parse.syntax()).unwrap();

        dbg!(root.exprs().collect::<Vec<_>>());
    }
}
