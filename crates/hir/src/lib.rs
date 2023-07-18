use syntax::SyntaxKind;

#[derive(Debug)]
pub enum Expr {
    BinaryExpr { op: BinaryOp, lhs: Box<Self>, rhs: Box<Self> },
    Literal { n: u64 },
    Missing,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
}

impl Expr {
    fn lower(ast: Option<ast::Expr>) -> Self {
        if let Some(ast) = ast {
            match ast {
                ast::Expr::BinaryExpr(ast) => Self::lower_binary(ast),
                ast::Expr::Literal(ast) => Self::Literal { n: ast.parse() },
            }
        } else {
            Self::Missing
        }
    }

    fn lower_binary(ast: ast::BinaryExpr) -> Self {
        let op = match ast.op().unwrap().kind() {
            SyntaxKind::Plus => BinaryOp::Add,
            _ => unreachable!(),
        };

        Self::BinaryExpr {
            op,
            lhs: Box::new(Expr::lower(ast.lhs())),
            rhs: Box::new(Expr::lower(ast.rhs())),
        }
    }
}

pub fn lower(ast: ast::Root) -> impl Iterator<Item = Expr> {
    ast.exprs().map(Expr::lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // snip
        let input = "1 + 2";
        let parse = syntax::clia_to_cst(&input);
        println!("{}", parse.debug_tree());

        let root = ast::Root::cast(parse.syntax()).unwrap();

        dbg!(lower(root).collect::<Vec<_>>());
    }
}
