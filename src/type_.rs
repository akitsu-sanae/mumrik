use context::Context;
use expr::{BinOp, Expr, Literal};

pub mod printer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Int,
    Bool,
    Char,
    Unit,
    Variable(String),
    Function(Box<Type>, Box<Type>),
    Record(Vec<(String, Type)>),
    Variant(Vec<(String, Type)>),
    List(Box<Type>),
}

// Note: `context` includes both value-variable bindings and type-variable bindings
pub fn check(expr: &Expr, context: &Context<Type>) -> Result<Type, String> {
    match *expr {
        Expr::Const(Literal::Number(_)) => Ok(Type::Int),
        Expr::Const(Literal::Bool(_)) => Ok(Type::Bool),
        Expr::Const(Literal::Char(_)) => Ok(Type::Char),
        Expr::Const(Literal::Unit) => Ok(Type::Unit),
        Expr::Const(Literal::List(ref exprs)) => {
            let mut element_ty = None;
            for expr in exprs {
                let expr_ty = check(expr, context)?;
                if element_ty.is_none() {
                    element_ty = Some(expr_ty);
                } else if element_ty.as_ref() != Some(&expr_ty) {
                    return Err(format!(
                        "nyan not match type: {:?} and {:?}",
                        element_ty.unwrap(),
                        expr_ty
                    ));
                }
            }
            Ok(Type::List(box element_ty.unwrap()))
        }
        Expr::Var(ref name) => context.lookup(name),
        Expr::Lambda(ref name, box ref ty, box ref e) => {
            let new_context = context.add(name, &ty);
            let ret_ty = check(e, &new_context)?;
            Ok(Type::Function(box ty.clone(), box ret_ty))
        }
        Expr::Apply(box ref e1, box ref e2) => {
            let param = check(e2, context)?;
            let f_ty = check(e1, context)?;
            if let Type::Function(box arg, box ret) = f_ty {
                if arg == param {
                    Ok(ret)
                } else {
                    Err(format!("not match type: {:?} and {:?}", arg, param))
                }
            } else {
                Err(format!("can not apply to non functional type: {:?}", f_ty))
            }
        }
        Expr::Let(ref name, box ref init, box ref body) => {
            let new_context = context.add(name, &check(init, context)?);
            check(body, &new_context)
        }
        Expr::LetRec(ref name, box ref ty, box ref init, box ref body) => {
            let new_context = context.add(name, &ty);
            if check(init, &new_context) == Ok(ty.clone()) {
                check(body, &new_context)
            } else {
                Err(format!("type error: not match {:?}", ty))
            }
        }
        Expr::LetType(ref name, box ref ty, box ref body) => {
            // Note: `name` is type-variable
            let new_context = context.add(name, ty);
            check(body, &new_context)
        }
        Expr::If(box ref cond, box ref tr, box ref fl) => {
            let cond_ty = check(cond, context)?;
            if cond_ty == Type::Bool {
                let tr_ty = check(tr, context)?;
                let fl_ty = check(fl, context)?;
                if tr_ty == fl_ty {
                    Ok(tr_ty)
                } else {
                    Err(format!("unmatch type: {:?} and {:?}", tr_ty, fl_ty))
                }
            } else {
                Err(format!("if condition must be bool"))
            }
        }
        Expr::BinOp(BinOp::Equal, box ref e1, box ref e2)
        | Expr::BinOp(BinOp::NotEqual, box ref e1, box ref e2) => {
            let e1_ty = check(e1, context)?;
            let e2_ty = check(e2, context)?;
            match (e1_ty, e2_ty) {
                (Type::Int, Type::Int) | (Type::Char, Type::Char) | (Type::Bool, Type::Bool) => {
                    Ok(Type::Bool)
                }
                (l, r) => Err(format!("can not compare {:?} and {:?}", l, r)),
            }
        }
        Expr::BinOp(BinOp::LessThan, box ref e1, box ref e2)
        | Expr::BinOp(BinOp::GreaterThan, box ref e1, box ref e2) => {
            let e1_ty = check(e1, context)?;
            let e2_ty = check(e2, context)?;
            match (e1_ty, e2_ty) {
                (Type::Int, Type::Int) | (Type::Char, Type::Char) => Ok(Type::Bool),
                (l, r) => Err(format!("can not compare {:?} and {:?}", l, r)),
            }
        }
        Expr::BinOp(BinOp::Add, box ref e1, box ref e2)
        | Expr::BinOp(BinOp::Sub, box ref e1, box ref e2)
        | Expr::BinOp(BinOp::Mult, box ref e1, box ref e2)
        | Expr::BinOp(BinOp::Div, box ref e1, box ref e2) => {
            let e1_ty = check(e1, context)?;
            let e2_ty = check(e2, context)?;
            if let (Type::Int, Type::Int) = (e1_ty, e2_ty) {
                Ok(Type::Int)
            } else {
                Err(format!("can not add non numeric values"))
            }
        }
        Expr::Const(Literal::Record(ref v)) => {
            let mut branches = vec![];
            for &(ref label, ref expr) in v {
                let ty = check(expr, context)?;
                branches.push((label.clone(), ty));
            }
            Ok(Type::Record(branches))
        }
        Expr::Const(Literal::Variant(ref tag, box ref e, box ref ty)) => {
            if let Type::Variant(v) = ty.clone() {
                let found = v.iter().find(|e| &e.0 == tag);
                if let Some(branch) = found {
                    let e_ty = check(e, context)?;
                    if e_ty == branch.1 {
                        Ok(ty.clone())
                    } else {
                        Err(format!(
                            "not much variant type: tag {} is related to {:?}, not {:?}",
                            branch.0, e_ty, branch.1
                        ))
                    }
                } else {
                    Err(format!("not found such tag {} in variant {:?}", tag, ty))
                }
            } else {
                Err(format!("variant type specifier must be variant type"))
            }
        }
        Expr::Dot(box ref e, ref label) => {
            if let Type::Record(branches) = check(e, context)? {
                match branches.into_iter().find(|e| &e.0 == label) {
                    Some(branch) => Ok(branch.1),
                    None => Err(format!("not found such filed in {:?} * {}", e, label)),
                }
            } else {
                Err(format!("can not apply dot operator for non record"))
            }
        }
        Expr::Match(box ref e, ref branches) => match_typecheck(e, branches, context),
        Expr::Println(box ref e) => {
            check(e, context)?;
            Ok(Type::Unit)
        }
    }
}

fn match_typecheck(
    e: &Expr,
    branches: &Vec<(String, String, Box<Expr>)>,
    context: &Context<Type>,
) -> Result<Type, String> {
    let e_ty = check(e, context)?;
    if let Type::Variant(v) = e_ty {
        let mut ret_ty = None;
        for (idx, (label, ty)) in v.into_iter().enumerate() {
            let ref branch = branches[idx];
            if label != branch.0 {
                return Err(format!("not match label: {} and {}", label, branch.0));
            }
            let new_context = context.add(&branch.1, &ty);
            let ty = check(&branch.2, &new_context)?;
            if ret_ty == None {
                ret_ty = Some(ty);
            } else if ret_ty != Some(ty) {
                return Err(format!("can not much all match return types"));
            }
        }
        if let Some(ty) = ret_ty {
            Ok(ty)
        } else {
            Err(format!("no branches"))
        }
    } else {
        Err(format!("type error: can not match for non variant expr"))
    }
}
