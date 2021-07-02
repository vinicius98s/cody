use std::path::Path;
use string_cache::Atom;
use swc_common::{self, sync::Lrc, SourceMap};
use swc_ecma_ast::{ExprStmt, MemberExpr};
use swc_ecma_parser::{lexer::{Lexer}, Capturing, Parser, StringInput, Syntax};

fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm
        .load_file(Path::new("foo.js"))
        .expect("failed to load foo.js");

    let lexer = Lexer::new(
        Syntax::default(),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);
    let mut parser = Parser::new_from(capturing);

    let script = parser.parse_script().unwrap();
    for stmt in script.body {
        if stmt.is_expr() {
            let expr = stmt.expr().unwrap();
            if is_console_log(expr) {
                println!("found console log!")
            }
        }
    }
}

fn is_console_log(expr_stmt: ExprStmt) -> bool {
    let expr = *expr_stmt.expr;

    match expr.call() {
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
