use super::*;
use ast::parsed;
use env::Env;

impl Type {
    // precondition: `typ` does not contain type variable.
    pub fn from_parsed_type(typ: &parsed::Type) -> Self {
        match typ {
            parsed::Type::Int(_) => Type::Int,
            parsed::Type::Bool(_) => Type::Bool,
            parsed::Type::Char(_) => Type::Char,
            parsed::Type::Unit(_) => Type::Unit,
            parsed::Type::Var(_, _) => unreachable!(),
            parsed::Type::Func(box ref typ1, box ref typ2, _) => Type::Func(
                box Self::from_parsed_type(typ1),
                box Self::from_parsed_type(typ2),
            ),
            parsed::Type::Record(fields, _) => Type::Record(
                fields
                    .iter()
                    .map(|(label, typ)| (label.clone(), Self::from_parsed_type(typ)))
                    .collect(),
            ),
            parsed::Type::Variant(ctors, _) => Type::Variant(
                ctors
                    .iter()
                    .map(|(label, typ)| (label.clone(), Self::from_parsed_type(typ)))
                    .collect(),
            ),
        }
    }
}

pub fn type_of(e: &Expr) -> Type {
    type_of_expr(e, &Env::new())
}

// NOTE: this function does *not* check well-typed or not
fn type_of_expr(e: &Expr, env: &Env<Type>) -> Type {
    match e {
        Expr::Const(lit) => type_of_literal(lit, env),
        Expr::Var(_, ref typ) => typ.clone(),
        Expr::Lambda(ref param_name, ref param_type, box ref e) => {
            let env = env.add(param_name.clone(), param_type.clone());
            Type::Func(box param_type.clone(), box type_of_expr(e, &env))
        }
        Expr::Apply(box ref e1, _) => {
            if let Type::Func(_, box ret_type) = type_of_expr(e1, env) {
                ret_type
            } else {
                unreachable!()
            }
        }
        Expr::LetRec(ref name, ref typ, _, box ref e) => {
            let env = env.add(name.clone(), typ.clone());
            type_of_expr(e, &env)
        }
        Expr::Let(ref name, box ref e1, box ref e2) => {
            let typ = type_of_expr(e1, env);
            let env = env.add(name.clone(), typ.clone());
            type_of_expr(e2, &env)
        }
        Expr::If(_, box ref e, _) => type_of_expr(e, env),
        Expr::BinOp(op, box ref e1, box ref e2) => {
            let (typ1, typ2) = (type_of_expr(e1, env), type_of_expr(e2, env));
            match (op, typ1, typ2) {
                (BinOp::Add, Type::Int, Type::Int)
                | (BinOp::Sub, Type::Int, Type::Int)
                | (BinOp::Mult, Type::Int, Type::Int)
                | (BinOp::Div, Type::Int, Type::Int) => Type::Int,
                (BinOp::Lt, Type::Int, Type::Int) | (BinOp::Gt, Type::Int, Type::Int) => Type::Bool,
                (BinOp::Eq, typ1, typ2) | (BinOp::Neq, typ1, typ2) if &typ1 == &typ2 => typ1,
                _ => unreachable!(),
            }
        }
        Expr::FieldAccess(box ref e, ref label) => {
            if let Type::Record(fields) = type_of_expr(e, env) {
                fields
                    .into_iter()
                    .find(|&(ref label_, _)| label == label_)
                    .unwrap()
                    .1
            } else {
                unreachable!()
            }
        }
        Expr::PatternMatch(box ref e, ref arms) => {
            if let Type::Variant(ctors) = type_of_expr(e, env) {
                let (ref label, ref typ) = ctors[0];
                let arm = arms.iter().find(|ref arm| &arm.label == label).unwrap();
                let env = env.add(arm.name.clone(), typ.clone());
                type_of_expr(&arm.body, &env)
            } else {
                unreachable!()
            }
        }
        Expr::Println(_) => Type::Unit,
    }
}

fn type_of_literal(lit: &Literal, env: &Env<Type>) -> Type {
    match lit {
        Literal::Number(_) => Type::Int,
        Literal::Bool(_) => Type::Bool,
        Literal::Char(_) => Type::Char,
        Literal::Unit => Type::Unit,
        Literal::Variant(_, _, typ) => typ.clone(),
        Literal::Record(fields) => Type::Record(
            fields
                .iter()
                .map(|(label, e)| (label.clone(), type_of_expr(e, env)))
                .collect(),
        ),
    }
}
