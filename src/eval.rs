use context::Context;
use expr::{
    BinOp::{self, *},
    Expr::{self, *},
    Literal::*,
};

fn binop(op: &BinOp, e1: &Expr, e2: &Expr) -> Result<Expr, String> {
    Ok(Const(match (op, e1, e2) {
        (&Equal, &Const(Number(ref n1)), &Const(Number(ref n2))) => Bool(n1 == n2),
        (&Equal, &Const(Bool(ref b1)), &Const(Bool(ref b2))) => Bool(b1 == b2),

        (&NotEqual, &Const(Number(ref n1)), &Const(Number(ref n2))) => Bool(n1 != n2),
        (&NotEqual, &Const(Bool(ref b1)), &Const(Bool(ref b2))) => Bool(b1 != b2),

        (&LessThan, &Const(Number(ref n1)), &Const(Number(ref n2))) => Bool(n1 < n2),
        (&GreaterThan, &Const(Number(ref n1)), &Const(Number(ref n2))) => Bool(n1 > n2),

        (&Add, &Const(Number(ref n1)), &Const(Number(ref n2))) => Number(n1 + n2),
        (&Sub, &Const(Number(ref n1)), &Const(Number(ref n2))) => Number(n1 - n2),
        (&Mult, &Const(Number(ref n1)), &Const(Number(ref n2))) => Number(n1 * n2),
        (&Div, &Const(Number(ref n1)), &Const(Number(ref n2))) => Number(n1 / n2),
        (op, e1, e2) => return Err(format!("cannot {} for {} and {}", op, e1, e2)),
    }))
}

pub fn expr(expr: &Expr, context: &Context<Expr>) -> Result<Expr, String> {
    match expr {
        Apply(box ref f, box ref arg) => match f {
            Lambda(ref name, _, box ref body) if arg.is_value() => {
                let new_context = context.add(name, arg);
                self::expr(body, &new_context)
            }
            Lambda(_, _, _) => {
                let arg = self::expr(arg, context)?;
                self::expr(&Apply(box f.clone(), box arg), context)
            }
            _ => {
                let f = self::expr(f, context)?;
                self::expr(&Apply(box f, box arg.clone()), context)
            }
        },
        Let(ref name, box ref init, box ref after) => {
            let new_context = context.add(name, init);
            self::expr(after, &new_context)
        }
        LetRec(ref name, _, box ref init, box ref body) => {
            let new_context = context.add(name, init);
            self::expr(body, &new_context)
        }
        LetType(_, _, box ref body) => self::expr(body, &context),
        If(box ref cond, box ref tr, box ref fl) => match self::expr(cond, context)? {
            Const(Bool(c)) => {
                if c {
                    self::expr(tr, context)
                } else {
                    self::expr(fl, context)
                }
            }
            _ => Err(format!("if condition must be bool: {:?}", cond)),
        },
        BinOp(op, box ref e1, box ref e2) => {
            self::binop(op, &self::expr(&e1, context)?, &self::expr(&e2, context)?)
        }
        Dot(box ref e, ref label) => match self::expr(e, context)? {
            Const(Record(v)) => {
                let found = v.iter().find(|e| e.0 == label.clone());
                if let Some(branch) = found {
                    Ok(*branch.1.clone())
                } else {
                    Err(format!("not found such filed in {:?} : {}", e, label))
                }
            }
            _ => Err(format!("can not apply dot operator for non record")),
        },
        Match(box ref e, ref branches) => match e {
            Const(Variant(ref label, box ref e, box ref ty)) => {
                let found = branches.iter().find(|br| label.clone() == br.0);
                if let Some(branch) = found {
                    let new_context = context.add(&branch.1, e);
                    self::expr(&branch.2, &new_context)
                } else {
                    Err(format!("can not find such label in {:?}: {}", ty, label))
                }
            }
            _ => Err(format!("can not apply match operator for non variant")),
        },
        Println(box ref e) => {
            let e = self::expr(e, context)?;
            println!("{}", e);
            Ok(e)
        }
        Var(ref name) => context.lookup(name),
        _ => Ok(expr.clone()),
    }
}
