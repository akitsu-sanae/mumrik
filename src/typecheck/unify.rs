use super::subst::Subst;
use super::*;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Constraint {
    Equation(Type, Type, Position),
    RecordAt(Type, Ident, Type, Position),
}

pub fn solve(constraints: VecDeque<Constraint>) -> Result<Subst, Error> {
    let mut queue = VecDeque::from(constraints);
    let mut subst = Subst::new();

    while !queue.is_empty() {
        match queue.pop_front().unwrap() {
            Constraint::Equation(typ1, typ2, _) if typ1 == typ2 => (),
            Constraint::Equation(
                Type::Func(box typ11, box typ12),
                Type::Func(box typ21, box typ22),
                pos,
            ) => {
                queue.push_back(Constraint::Equation(typ11, typ21, pos.clone()));
                queue.push_back(Constraint::Equation(typ12, typ22, pos));
            }
            Constraint::Equation(Type::Record(fields1), Type::Record(fields2), pos) => {
                let fields1_keys: Vec<_> = fields1.keys().collect();
                let fields2_keys: Vec<_> = fields2.keys().collect();
                if fields1_keys != fields2_keys {
                    return Err(Error::Unify {
                        pos: pos.clone(),
                        typ1: Type::Record(fields1),
                        typ2: Type::Record(fields2),
                    });
                }
                for (field1, field2) in fields1.into_iter().zip(fields2.into_iter()) {
                    queue.push_back(Constraint::Equation(field1.1, field2.1, pos.clone()));
                }
            }
            Constraint::Equation(Type::Var(name), typ, pos)
            | Constraint::Equation(typ, Type::Var(name), pos) => match subst.0.get(&name) {
                Some(typ_) => {
                    queue.push_back(Constraint::Equation(typ, typ_.clone(), pos.clone()));
                }
                None => {
                    if typ.is_occurs(&name) {
                        return Err(Error::RecOccur {
                            pos: pos.clone(),
                            var: name,
                            typ: typ,
                        });
                    }

                    let typ = subst.apply_type(typ);
                    subst = Subst(
                        subst
                            .0
                            .into_iter()
                            .map(|(name_, typ_)| (name_, typ_.subst_type(&name, &typ)))
                            .collect(),
                    );
                    subst.0.insert(name, typ);
                }
            },
            Constraint::Equation(typ1, typ2, pos) => {
                return Err(Error::Unify {
                    pos: pos,
                    typ1,
                    typ2,
                });
            }
            Constraint::RecordAt(typ1, label, typ2, pos) => match subst.apply_type(typ1) {
                typ1 @ Type::Record(_) => {
                    let typ1_str = format!("{:?}", typ1);
                    let fields = if let Type::Record(fields) = typ1 {
                        fields
                    } else {
                        unreachable!()
                    };
                    let elem_typ = match fields.into_iter().find(|(ref label_, _)| label_ == &label)
                    {
                        Some((_, typ)) => typ,
                        None => {
                            return Err(Error::Other {
                                pos,
                                message: format!("`{}` does not have field `{}`", typ1_str, label),
                            })
                        }
                    };
                    queue.push_back(Constraint::Equation(elem_typ, typ2, pos));
                }
                Type::Var(name) => {
                    queue.push_back(Constraint::RecordAt(Type::Var(name), label, typ2, pos));
                }
                typ1 => {
                    return Err(Error::Other {
                        pos,
                        message: format!("`{}` cannnot be indexed with label `{}`", typ1, label),
                    });
                }
            },
        }
    }

    Ok(subst)
}
