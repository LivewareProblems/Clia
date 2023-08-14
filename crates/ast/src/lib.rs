use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};

#[derive(Debug)]
pub struct Root(SyntaxNode);

impl Root {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Source {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn exprs(&self) -> impl Iterator<Item = Option<Expr>> {
        self.0.children().map(Expr::cast)
    }
}

#[derive(Debug)]
pub enum Expr {
    BinaryExpr(BinaryExpr),
    Literal(Literal),
}
impl Expr {
    pub fn cast(node: SyntaxNode) -> Option<Self> {
        let result = match node.kind() {
            SyntaxKind::BinaryOp => Self::BinaryExpr(BinaryExpr(node)),
            SyntaxKind::Literal => Self::Literal(Literal(node)),
            _ => return None,
        };

        Some(result)
    }
}
#[derive(Debug)]
pub struct Literal(SyntaxNode);

impl Literal {
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
            .find(|token| matches!(token.kind(), SyntaxKind::Plus,))
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

    #[test]
    fn it_works_with_comments() {
        let input = "1 + 2 # a comment";
        let parse = clia_to_cst(&input);
        println!("{}", parse.debug_tree());

        let root = Root::cast(parse.syntax()).unwrap();

        dbg!(root.exprs().collect::<Vec<_>>());
    }
}
