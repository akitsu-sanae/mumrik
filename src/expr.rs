use std::collections::HashMap;
use type_::Type;

pub mod eval;
pub mod parser;
pub mod printer;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Number(i32),
    Bool(bool),
    Char(char),
    Unit,
    List(Vec<Expr>),
    Var(String),
    Lambda(String, Box<Type>, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Sequence(Box<Expr>, Box<Expr>),
    Let(String, Box<Expr>, Box<Expr>),
    LetRec(String, Box<Type>, Box<Expr>, Box<Expr>),
    LetType(String, Box<Type>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Record(Vec<(String, Box<Expr>)>),
    Dot(Box<Expr>, String),
    Variant(String, Box<Expr>, Box<Type>),
    // match expr {
    //     Hoge x => x + 1,
    //     Fuga x => x * 3,
    // }
    Match(Box<Expr>, Vec<(String, String, Box<Expr>)>),
    Println(Box<Expr>),
}

impl Expr {
    pub fn subst_typealias(&mut self, alias: &HashMap<String, Type>) {
        use expr::Expr::*;
        match *self {
            List(ref mut exprs) => {
                for expr in exprs {
                    expr.subst_typealias(alias);
                }
            }
            Lambda(_, box ref mut ty, box ref mut expr)
            | Variant(_, box ref mut expr, box ref mut ty) => {
                ty.subst(alias);
                expr.subst_typealias(alias)
            }
            LetRec(_, box ref mut ty, box ref mut e, box ref mut body) => {
                ty.subst(alias);
                e.subst_typealias(alias);
                body.subst_typealias(alias);
            }
            If(box ref mut cond, box ref mut tr, box ref mut fl) => {
                cond.subst_typealias(alias);
                tr.subst_typealias(alias);
                fl.subst_typealias(alias);
            }
            LetType(ref name, box ref ty, box ref mut e) => {
                let mut alias = alias.clone();
                alias.insert(name.to_string(), ty.clone());
                e.subst_typealias(&alias);
            }
            Let(_, box ref mut e1, box ref mut e2)
            | Apply(box ref mut e1, box ref mut e2)
            | Sequence(box ref mut e1, box ref mut e2)
            | BinOp(_, box ref mut e1, box ref mut e2) => {
                e1.subst_typealias(alias);
                e2.subst_typealias(alias);
            }
            Dot(box ref mut e, _) | Println(box ref mut e) => e.subst_typealias(alias),
            Record(ref mut params) => {
                for &mut (_, ref mut e) in params.iter_mut() {
                    e.subst_typealias(alias);
                }
            }
            Match(box ref mut e, ref mut branches) => {
                e.subst_typealias(alias);
                for &mut (_, _, box ref mut e) in branches {
                    e.subst_typealias(alias)
                }
            }
            Number(_) | Bool(_) | Char(_) | Unit | Var(_) => (),
        }
    }

    fn is_value(&self) -> bool {
        match self {
            &Expr::Number(_) | &Expr::Bool(_) | &Expr::Char(_) => true,
            &Expr::Unit => true,
            &Expr::Lambda(_, _, _) => true,
            &Expr::Record(_) => true,
            &Expr::Variant(_, _, _) => true,
            &Expr::List(_) => true,
            _ => false,
        }
    }
}
