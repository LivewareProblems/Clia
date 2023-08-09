use core::panic;

use ast;
use hir::{BinaryOp, Expr};
use janetrs::client::{Error, JanetClient};

pub fn run_janet_code(text: &str) -> Result<(), Error> {
    let client = JanetClient::init_with_default_env()?;

    let out = client.run(text)?;

    println!("{out}");

    Ok(())
}

pub fn ast_to_janet(ast: ast::Root) -> String {
    let hir_root = hir::lower(ast);
    hir_root.map(|x| hir_expr_to_janet(x)).reduce(|x, y| x + &y).unwrap()
}

fn hir_expr_to_janet(expr: Expr) -> String {
    match expr {
        Expr::BinaryExpr { op, lhs, rhs } => {
            let op_string = hir_op_to_janet(op);
            let lhs_string = hir_expr_to_janet(*lhs);
            let rhs_string = hir_expr_to_janet(*rhs);
            return format!("( {op_string} {lhs_string} {rhs_string} )");
        }
        Expr::Literal { n } => return format!("{n}"),
        Expr::Missing => panic!("wrong HIR"),
    }
}

fn hir_op_to_janet(bin: BinaryOp) -> String {
    match bin {
        BinaryOp::Add => return "+".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = run_janet_code("(print `Hello from Janet!`)");
    }

    #[test]
    fn end_to_end() {
        let input = "1 + 2";
        let parse = syntax::clia_to_cst(&input);
        let root = ast::Root::cast(parse.syntax()).unwrap();
        let janet_code = ast_to_janet(root);

        println!("{}", janet_code);
        let _ = run_janet_code(&janet_code);
    }
}
