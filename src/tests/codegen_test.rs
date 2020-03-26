use codegen::codegen;
use expr::{
    Expr::{self, *},
    Literal::*,
};

fn check(e: Expr, expected: &str, filename: &str) {
    let filename = format!("./test/{}", filename);
    codegen(
        Let(
            "<dummy>".to_string(),
            box Println(box e),
            box Const(Number(0)),
        ),
        filename.as_str(),
    );
    {
        use std::process::Command;
        // run & check
        let result = Command::new("lli")
            .arg(&filename)
            .output()
            .expect("failed to execute lli");
        let output = std::str::from_utf8(&result.stdout).expect("unrecognazed outut");
        assert_eq!(output, expected);
        assert!(result.status.success());
    }
}

#[test]
fn const_() {
    let e = Const(Number(42));
    check(e, "42\n", "const.ll");
}

#[test]
fn let_() {
    let e = Let(
        "a".to_string(),
        box Const(Number(42)),
        box Var("a".to_string()),
    );
    check(e, "42\n", "let.ll");
}

#[test]
fn if_() {
    let e = If(
        box Const(Bool(true)),
        box Const(Number(1)),
        box Const(Number(2)),
    );
    check(e, "1\n", "if.ll");
}
