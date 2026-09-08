use crate::{ast::*, error::Error};

/// 式を評価して整数または真偽値を返します。
/// 整数の範囲を検査し、ifは選択された分岐だけ評価します。
pub fn eval(expr: &Exp) -> Result<Value, Error> {
    match expr {
        Exp::Literal(Value::Bool(b)) => Ok(Value::Bool(*b)),
        Exp::Literal(Value::Int(i)) => Ok(Value::Int(*i)),
        Exp::Binary(e1, op, e2) => {
            let v1 = eval(e1)?;
            let v2 = eval(e2)?;
            match (v1, v2) {
                (Value::Int(i1), Value::Int(i2)) => match op {
                    Prim::Add => i1.checked_add(i2).map(Value::Int).ok_or(Error::Overflow),
                    Prim::Sub => i1.checked_sub(i2).map(Value::Int).ok_or(Error::Overflow),
                    Prim::Mul => i1.checked_mul(i2).map(Value::Int).ok_or(Error::Overflow),
                    Prim::Lt => Ok(Value::Bool(i1 < i2)),
                },
                _ => Err(Error::Type("not a number".to_string())),
            }
        }
        Exp::If(cond, then_clause, else_clause) => {
            let v = eval(cond)?;
            match v {
                Value::Bool(b) => {
                    if b {
                        let v1 = eval(then_clause)?;
                        Ok(v1)
                    } else {
                        let v2 = eval(else_clause)?;
                        Ok(v2)
                    }
                }
                _ => Err(Error::Type("not a boolean value".to_string())),
            }
        }
    }
}
