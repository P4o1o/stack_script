//
// math_op.rs - Mathematical operations
//

use crate::error::{Result, SscriptError};
use crate::stack::{Stack, StackElem};

pub fn op_size(stack: &mut Stack) {
    stack.push(StackElem::Integer(stack.len() as i64));
}

pub fn op_int(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    match elem {
        StackElem::Integer(i) => {
            stack.push(StackElem::Integer(i));
            Ok(())
        }
        StackElem::Floating(f) => {
            stack.push(StackElem::Integer(f as i64));
            Ok(())
        }
        _ => {
            stack.push(elem);
            Err(SscriptError::InvalidOperands("int: expected numeric".into()))
        }
    }
}

pub fn op_add(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Integer(ai), StackElem::Integer(bi)) => {
            stack.push(StackElem::Integer(ai + bi));
        }
        (StackElem::Floating(af), StackElem::Integer(bi)) => {
            stack.push(StackElem::Floating(af + *bi as f64));
        }
        (StackElem::Integer(ai), StackElem::Floating(bf)) => {
            stack.push(StackElem::Floating(*ai as f64 + bf));
        }
        (StackElem::Floating(af), StackElem::Floating(bf)) => {
            stack.push(StackElem::Floating(af + bf));
        }
        _ => {
            stack.push(a);
            stack.push(b);
            return Err(SscriptError::InvalidOperands("+: expected numeric operands".into()));
        }
    }
    Ok(())
}

pub fn op_sub(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Integer(ai), StackElem::Integer(bi)) => {
            stack.push(StackElem::Integer(ai - bi));
        }
        (StackElem::Floating(af), StackElem::Integer(bi)) => {
            stack.push(StackElem::Floating(af - *bi as f64));
        }
        (StackElem::Integer(ai), StackElem::Floating(bf)) => {
            stack.push(StackElem::Floating(*ai as f64 - bf));
        }
        (StackElem::Floating(af), StackElem::Floating(bf)) => {
            stack.push(StackElem::Floating(af - bf));
        }
        _ => {
            stack.push(a);
            stack.push(b);
            return Err(SscriptError::InvalidOperands("-: expected numeric operands".into()));
        }
    }
    Ok(())
}

pub fn op_mul(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Integer(ai), StackElem::Integer(bi)) => {
            stack.push(StackElem::Integer(ai * bi));
        }
        (StackElem::Floating(af), StackElem::Integer(bi)) => {
            stack.push(StackElem::Floating(af * *bi as f64));
        }
        (StackElem::Integer(ai), StackElem::Floating(bf)) => {
            stack.push(StackElem::Floating(*ai as f64 * bf));
        }
        (StackElem::Floating(af), StackElem::Floating(bf)) => {
            stack.push(StackElem::Floating(af * bf));
        }
        _ => {
            stack.push(a);
            stack.push(b);
            return Err(SscriptError::InvalidOperands("*: expected numeric operands".into()));
        }
    }
    Ok(())
}

pub fn op_div(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    let bf = match &b {
        StackElem::Integer(i) => *i as f64,
        StackElem::Floating(f) => *f,
        _ => {
            stack.push(a);
            stack.push(b);
            return Err(SscriptError::InvalidOperands("/: expected numeric operands".into()));
        }
    };
    
    if bf == 0.0 {
        stack.push(a);
        stack.push(b);
        return Err(SscriptError::ValueError("Division by zero".into()));
    }
    
    let af = match &a {
        StackElem::Integer(i) => *i as f64,
        StackElem::Floating(f) => *f,
        _ => {
            stack.push(a);
            stack.push(b);
            return Err(SscriptError::InvalidOperands("/: expected numeric operands".into()));
        }
    };
    
    stack.push(StackElem::Floating(af / bf));
    Ok(())
}

pub fn op_mod(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    match (&a, &b) {
        (StackElem::Integer(ai), StackElem::Integer(bi)) => {
            if *bi == 0 {
                stack.push(a);
                stack.push(b);
                return Err(SscriptError::ValueError("Modulo by zero".into()));
            }
            stack.push(StackElem::Integer(ai % bi));
            Ok(())
        }
        _ => {
            stack.push(a);
            stack.push(b);
            Err(SscriptError::InvalidOperands("%: expected integer operands".into()))
        }
    }
}

pub fn op_pow(stack: &mut Stack) -> Result<()> {
    let b = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let a = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    
    let bf = match &b {
        StackElem::Integer(i) => *i as f64,
        StackElem::Floating(f) => *f,
        _ => {
            stack.push(a);
            stack.push(b);
            return Err(SscriptError::InvalidOperands("pow: expected numeric operands".into()));
        }
    };
    
    let af = match &a {
        StackElem::Integer(i) => *i as f64,
        StackElem::Floating(f) => *f,
        _ => {
            stack.push(a);
            stack.push(b);
            return Err(SscriptError::InvalidOperands("pow: expected numeric operands".into()));
        }
    };
    
    stack.push(StackElem::Floating(af.powf(bf)));
    Ok(())
}

macro_rules! unary_float_op {
    ($stack:expr, $op:ident, $name:expr) => {{
        let elem = $stack.pop().ok_or(SscriptError::StackUnderflow)?;
        let f = match &elem {
            StackElem::Integer(i) => *i as f64,
            StackElem::Floating(f) => *f,
            _ => {
                $stack.push(elem);
                return Err(SscriptError::InvalidOperands(format!(
                    "{}: expected numeric",
                    $name
                )));
            }
        };
        $stack.push(StackElem::Floating(f.$op()));
        Ok(())
    }};
}

pub fn op_sqrt(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let f = match &elem {
        StackElem::Integer(i) => *i as f64,
        StackElem::Floating(f) => *f,
        _ => {
            stack.push(elem);
            return Err(SscriptError::InvalidOperands("sqrt: expected numeric".into()));
        }
    };
    
    if f < 0.0 {
        stack.push(elem);
        return Err(SscriptError::ValueError("sqrt of negative number".into()));
    }
    
    stack.push(StackElem::Floating(f.sqrt()));
    Ok(())
}

pub fn op_exp(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, exp, "exp")
}

pub fn op_log(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, ln, "log")
}

pub fn op_log2(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, log2, "log2")
}

pub fn op_log10(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, log10, "log10")
}

pub fn op_opposite(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    match elem {
        StackElem::Integer(i) => stack.push(StackElem::Integer(-i)),
        StackElem::Floating(f) => stack.push(StackElem::Floating(-f)),
        _ => {
            stack.push(elem);
            return Err(SscriptError::InvalidOperands("--: expected numeric".into()));
        }
    }
    Ok(())
}

pub fn op_sin(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, sin, "sin")
}

pub fn op_cos(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, cos, "cos")
}

pub fn op_tan(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, tan, "tan")
}

pub fn op_arcsin(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, asin, "arcsin")
}

pub fn op_arccos(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, acos, "arccos")
}

pub fn op_arctan(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, atan, "arctan")
}

pub fn op_sinh(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, sinh, "sinh")
}

pub fn op_cosh(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, cosh, "cosh")
}

pub fn op_tanh(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, tanh, "tanh")
}

pub fn op_arcsinh(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, asinh, "arcsinh")
}

pub fn op_arccosh(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, acosh, "arccosh")
}

pub fn op_arctanh(stack: &mut Stack) -> Result<()> {
    unary_float_op!(stack, atanh, "arctanh")
}

// Fast factorial using the odd-factorial algorithm
fn factorial(n: i64) -> f64 {
    if n < 0 {
        return f64::NAN;
    }
    if n < 2 {
        return 1.0;
    }
    if n < 21 {
        // Direct computation for small values
        let mut result = 1u64;
        for i in 2..=n as u64 {
            result *= i;
        }
        return result as f64;
    }
    
    // For larger values, use iterative approach
    let mut result = 1.0f64;
    for i in 2..=n {
        result *= i as f64;
    }
    result
}

pub fn op_factorial(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    match elem {
        StackElem::Integer(i) => {
            if i < 0 {
                stack.push(StackElem::Integer(i));
                return Err(SscriptError::ValueError("Factorial of negative number".into()));
            }
            stack.push(StackElem::Floating(factorial(i)));
            Ok(())
        }
        _ => {
            stack.push(elem);
            Err(SscriptError::InvalidOperands("!: expected integer".into()))
        }
    }
}

pub fn op_gamma(stack: &mut Stack) -> Result<()> {
    let elem = stack.pop().ok_or(SscriptError::StackUnderflow)?;
    let f = match &elem {
        StackElem::Integer(i) => *i as f64,
        StackElem::Floating(f) => *f,
        _ => {
            stack.push(elem);
            return Err(SscriptError::InvalidOperands("gamma: expected numeric".into()));
        }
    };
    
    // Use Lanczos approximation for gamma function
    stack.push(StackElem::Floating(gamma_lanczos(f)));
    Ok(())
}

// Lanczos approximation for gamma function
fn gamma_lanczos(x: f64) -> f64 {
    if x <= 0.0 && x.fract() == 0.0 {
        return f64::INFINITY;
    }
    
    let g = 7;
    let c = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];
    
    if x < 0.5 {
        std::f64::consts::PI / ((std::f64::consts::PI * x).sin() * gamma_lanczos(1.0 - x))
    } else {
        let x = x - 1.0;
        let mut a = c[0];
        for i in 1..g + 2 {
            a += c[i] / (x + i as f64);
        }
        let t = x + g as f64 + 0.5;
        (2.0 * std::f64::consts::PI).sqrt() * t.powf(x + 0.5) * (-t).exp() * a
    }
}
