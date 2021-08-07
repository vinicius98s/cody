use string_cache::Atom;
use std::path::Path;
use swc_common::{self, sync::Lrc, SourceMap};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput};
use swc_ecma_ast::*;

fn main() {
    let cm: Lrc<SourceMap> = Default::default();

    let fm = cm
        .load_file(Path::new("foo.js"))
        .expect("faile to load foo.js");

    let lexer = Lexer::new(
        Default::default(),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = Parser::new_from(lexer);

    let script = parser.parse_script().expect("Failed to parse script");
    for stmt in script.body {
        if stmt.is_expr() {
            let expr = stmt.expr().unwrap();
            if is_console_log(&expr) {
                println!("found console log")
            }
        }
    }
}

fn is_console_log(expr_stmt: &ExprStmt) -> bool {
    match expr_stmt.clone().expr.call() {
        Some(call) => match call.callee.expr() {
            Some(expr) => match expr.member() {
                Some(member) => is_console_obj_expr(&member) && is_log_prop_expr(&member),
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

fn is_console_obj_expr(expr: &MemberExpr) -> bool {
    match expr.obj.clone().expr() {
        Some(member_expr) => match member_expr.ident() {
            Some(ident) => ident.sym == Atom::from("console"),
            _ => false,
        },
        _ => false,
    }
}

fn is_log_prop_expr(expr: &MemberExpr) -> bool {
    match expr.prop.clone().ident() {
        Some(ident) => ident.sym == Atom::from("log"),
        _ => false,
    }
}
