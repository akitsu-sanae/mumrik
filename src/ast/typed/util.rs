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
        _ => todo!(),
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
