// tests/integration_tests.rs - Comprehensive test suite for sscript

use sscript::interpreter::{execute, ProgramState};
use sscript::stack::StackElem;

// === Test Helpers ===

fn run_and_get_stack(code: &str) -> Vec<StackElem> {
    let mut state = ProgramState::new();
    execute(&mut state, code).unwrap_or_else(|e| panic!("Failed to execute '{}': {}", code, e));
    state.stack.content
}

fn run_and_get_top(code: &str) -> StackElem {
    let stack = run_and_get_stack(code);
    stack.last().unwrap_or_else(|| panic!("Stack empty after: {}", code)).clone()
}

fn run_expect_error(code: &str) -> bool {
    let mut state = ProgramState::new();
    execute(&mut state, code).is_err()
}

// ==================== LITERAL TESTS ====================

#[test]
fn test_push_integer() {
    let top = run_and_get_top("42");
    assert!(matches!(top, StackElem::Integer(42)));
}

#[test]
fn test_push_negative_integer() {
    let top = run_and_get_top("-42");
    assert!(matches!(top, StackElem::Integer(-42)));
}

#[test]
fn test_push_zero() {
    let top = run_and_get_top("0");
    assert!(matches!(top, StackElem::Integer(0)));
}

#[test]
fn test_push_float() {
    if let StackElem::Floating(f) = run_and_get_top("3.14") {
        assert!((f - 3.14).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_push_negative_float() {
    if let StackElem::Floating(f) = run_and_get_top("-3.14") {
        assert!((f + 3.14).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_push_string() {
    let top = run_and_get_top("\"hello world\"");
    if let StackElem::String(s) = top {
        assert_eq!(s.as_ref(), "hello world");
    } else {
        panic!("Expected string");
    }
}

#[test]
fn test_push_empty_string() {
    let top = run_and_get_top("\"\"");
    if let StackElem::String(s) = top {
        assert_eq!(s.as_ref(), "");
    } else {
        panic!("Expected string");
    }
}

#[test]
fn test_push_quoted_instruction() {
    let top = run_and_get_top("[2 3 +]");
    assert!(matches!(top, StackElem::Instruction(_)));
}

#[test]
fn test_push_nested_quoted() {
    let top = run_and_get_top("[[1 2] [3 4]]");
    assert!(matches!(top, StackElem::Instruction(_)));
}

#[test]
fn test_push_none() {
    let top = run_and_get_top("none");
    assert!(matches!(top, StackElem::None));
}

#[test]
fn test_push_multiple_values() {
    let stack = run_and_get_stack("1 2 3 4 5");
    assert_eq!(stack.len(), 5);
}

// ==================== ARITHMETIC OPERATIONS ====================

#[test]
fn test_add_integers() {
    assert!(matches!(run_and_get_top("5 3 +"), StackElem::Integer(8)));
}

#[test]
fn test_add_floats() {
    if let StackElem::Floating(f) = run_and_get_top("5.5 3.5 +") {
        assert!((f - 9.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_add_mixed() {
    if let StackElem::Floating(f) = run_and_get_top("5 3.5 +") {
        assert!((f - 8.5).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_subtract() {
    assert!(matches!(run_and_get_top("10 3 -"), StackElem::Integer(7)));
}

#[test]
fn test_subtract_negative_result() {
    assert!(matches!(run_and_get_top("3 10 -"), StackElem::Integer(-7)));
}

#[test]
fn test_multiply() {
    assert!(matches!(run_and_get_top("6 7 *"), StackElem::Integer(42)));
}

#[test]
fn test_multiply_by_zero() {
    assert!(matches!(run_and_get_top("100 0 *"), StackElem::Integer(0)));
}

#[test]
fn test_divide() {
    if let StackElem::Floating(f) = run_and_get_top("15 3 /") {
        assert!((f - 5.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_divide_by_zero_error() {
    assert!(run_expect_error("10 0 /"));
}

#[test]
fn test_modulo() {
    assert!(matches!(run_and_get_top("17 5 %"), StackElem::Integer(2)));
}

#[test]
fn test_modulo_exact() {
    assert!(matches!(run_and_get_top("15 5 %"), StackElem::Integer(0)));
}

#[test]
fn test_modulo_by_zero_error() {
    assert!(run_expect_error("10 0 %"));
}

#[test]
fn test_pow() {
    if let StackElem::Floating(f) = run_and_get_top("2 10 pow") {
        assert!((f - 1024.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_pow_zero_exponent() {
    if let StackElem::Floating(f) = run_and_get_top("5 0 pow") {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_sqrt() {
    if let StackElem::Floating(f) = run_and_get_top("9 sqrt") {
        assert!((f - 3.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_sqrt_float() {
    if let StackElem::Floating(f) = run_and_get_top("2.0 sqrt") {
        assert!((f - 1.414).abs() < 0.01);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_exp() {
    if let StackElem::Floating(f) = run_and_get_top("1 exp") {
        assert!((f - std::f64::consts::E).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_exp_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 exp") {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_log() {
    if let StackElem::Floating(f) = run_and_get_top("2.718281828 log") {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_log2() {
    if let StackElem::Floating(f) = run_and_get_top("8 log2") {
        assert!((f - 3.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_log10() {
    if let StackElem::Floating(f) = run_and_get_top("100 log10") {
        assert!((f - 2.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_opposite_int() {
    assert!(matches!(run_and_get_top("5 --"), StackElem::Integer(-5)));
}

#[test]
fn test_opposite_negative() {
    assert!(matches!(run_and_get_top("-5 --"), StackElem::Integer(5)));
}

#[test]
fn test_opposite_float() {
    if let StackElem::Floating(f) = run_and_get_top("3.14 --") {
        assert!((f + 3.14).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_factorial() {
    if let StackElem::Floating(f) = run_and_get_top("5 !") {
        assert!((f - 120.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_factorial_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 !") {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_factorial_one() {
    if let StackElem::Floating(f) = run_and_get_top("1 !") {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_gamma() {
    if let StackElem::Floating(f) = run_and_get_top("5 gamma") {
        // gamma(5) = 4! = 24
        assert!((f - 24.0).abs() < 0.1);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_int_cast() {
    assert!(matches!(run_and_get_top("3.9 int"), StackElem::Integer(3)));
}

#[test]
fn test_int_cast_negative() {
    assert!(matches!(run_and_get_top("-3.9 int"), StackElem::Integer(-3)));
}

// ==================== TRIGONOMETRIC OPERATIONS ====================

#[test]
fn test_sin_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 sin") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_cos_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 cos") {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_tan_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 tan") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_arcsin_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 arcsin") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_arccos_one() {
    if let StackElem::Floating(f) = run_and_get_top("1 arccos") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_arctan_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 arctan") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_sinh_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 sinh") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_cosh_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 cosh") {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_tanh_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 tanh") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_arcsinh_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 arcsinh") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_arccosh_one() {
    if let StackElem::Floating(f) = run_and_get_top("1 arccosh") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_arctanh_zero() {
    if let StackElem::Floating(f) = run_and_get_top("0 arctanh") {
        assert!(f.abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

// ==================== BOOLEAN OPERATIONS ====================

#[test]
fn test_true() {
    assert!(matches!(run_and_get_top("true"), StackElem::Boolean(true)));
}

#[test]
fn test_false() {
    assert!(matches!(run_and_get_top("false"), StackElem::Boolean(false)));
}

#[test]
fn test_and_tt() {
    assert!(matches!(run_and_get_top("true true and"), StackElem::Boolean(true)));
}

#[test]
fn test_and_tf() {
    assert!(matches!(run_and_get_top("true false and"), StackElem::Boolean(false)));
}

#[test]
fn test_and_ff() {
    assert!(matches!(run_and_get_top("false false and"), StackElem::Boolean(false)));
}

#[test]
fn test_or_tf() {
    assert!(matches!(run_and_get_top("true false or"), StackElem::Boolean(true)));
}

#[test]
fn test_or_ff() {
    assert!(matches!(run_and_get_top("false false or"), StackElem::Boolean(false)));
}

#[test]
fn test_or_tt() {
    assert!(matches!(run_and_get_top("true true or"), StackElem::Boolean(true)));
}

#[test]
fn test_xor_tt() {
    assert!(matches!(run_and_get_top("true true xor"), StackElem::Boolean(false)));
}

#[test]
fn test_xor_tf() {
    assert!(matches!(run_and_get_top("true false xor"), StackElem::Boolean(true)));
}

#[test]
fn test_xor_ff() {
    assert!(matches!(run_and_get_top("false false xor"), StackElem::Boolean(false)));
}

#[test]
fn test_not_true() {
    assert!(matches!(run_and_get_top("true not"), StackElem::Boolean(false)));
}

#[test]
fn test_not_false() {
    assert!(matches!(run_and_get_top("false not"), StackElem::Boolean(true)));
}

// ==================== COMPARISON OPERATIONS ====================

#[test]
fn test_eq_true() {
    assert!(matches!(run_and_get_top("5 5 =="), StackElem::Boolean(true)));
}

#[test]
fn test_eq_false() {
    assert!(matches!(run_and_get_top("5 3 =="), StackElem::Boolean(false)));
}

#[test]
fn test_eq_float_int() {
    assert!(matches!(run_and_get_top("5.0 5 =="), StackElem::Boolean(true)));
}

#[test]
fn test_neq_true() {
    assert!(matches!(run_and_get_top("5 3 !="), StackElem::Boolean(true)));
}

#[test]
fn test_neq_false() {
    assert!(matches!(run_and_get_top("5 5 !="), StackElem::Boolean(false)));
}

#[test]
fn test_lt_true() {
    assert!(matches!(run_and_get_top("3 5 <"), StackElem::Boolean(true)));
}

#[test]
fn test_lt_false() {
    assert!(matches!(run_and_get_top("5 3 <"), StackElem::Boolean(false)));
}

#[test]
fn test_lt_equal() {
    assert!(matches!(run_and_get_top("5 5 <"), StackElem::Boolean(false)));
}

#[test]
fn test_gt_true() {
    assert!(matches!(run_and_get_top("5 3 >"), StackElem::Boolean(true)));
}

#[test]
fn test_gt_false() {
    assert!(matches!(run_and_get_top("3 5 >"), StackElem::Boolean(false)));
}

#[test]
fn test_lte_less() {
    assert!(matches!(run_and_get_top("3 5 <="), StackElem::Boolean(true)));
}

#[test]
fn test_lte_equal() {
    assert!(matches!(run_and_get_top("5 5 <="), StackElem::Boolean(true)));
}

#[test]
fn test_lte_greater() {
    assert!(matches!(run_and_get_top("5 3 <="), StackElem::Boolean(false)));
}

#[test]
fn test_gte_greater() {
    assert!(matches!(run_and_get_top("5 3 >="), StackElem::Boolean(true)));
}

#[test]
fn test_gte_equal() {
    assert!(matches!(run_and_get_top("5 5 >="), StackElem::Boolean(true)));
}

#[test]
fn test_gte_less() {
    assert!(matches!(run_and_get_top("3 5 >="), StackElem::Boolean(false)));
}

// ==================== STACK OPERATIONS ====================

#[test]
fn test_dup() {
    let stack = run_and_get_stack("42 dup");
    assert_eq!(stack.len(), 2);
    assert!(matches!(&stack[0], StackElem::Integer(42)));
    assert!(matches!(&stack[1], StackElem::Integer(42)));
}

#[test]
fn test_dup0() {
    let stack = run_and_get_stack("42 dup0");
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_dup1() {
    let stack = run_and_get_stack("1 2 3 dup1");
    assert_eq!(stack.len(), 4);
    assert!(matches!(&stack[3], StackElem::Integer(2)));
}

#[test]
fn test_dup2() {
    let stack = run_and_get_stack("1 2 3 4 dup2");
    assert_eq!(stack.len(), 5);
    assert!(matches!(&stack[4], StackElem::Integer(2)));
}

#[test]
fn test_dup_bracket() {
    let stack = run_and_get_stack("1 2 3 4 dup(2)");
    assert_eq!(stack.len(), 5);
    assert!(matches!(&stack[4], StackElem::Integer(2)));
}

#[test]
fn test_dup_bracket_expr() {
    let stack = run_and_get_stack("1 2 3 dup(size 1 -)");
    assert_eq!(stack.len(), 4);
    assert!(matches!(&stack[3], StackElem::Integer(1)));
}

#[test]
fn test_swap() {
    let stack = run_and_get_stack("1 2 swap");
    assert!(matches!(&stack[0], StackElem::Integer(2)));
    assert!(matches!(&stack[1], StackElem::Integer(1)));
}

#[test]
fn test_swap1() {
    let stack = run_and_get_stack("1 2 swap1");
    assert!(matches!(&stack[0], StackElem::Integer(2)));
    assert!(matches!(&stack[1], StackElem::Integer(1)));
}

#[test]
fn test_swap2() {
    let stack = run_and_get_stack("1 2 3 swap2");
    assert!(matches!(&stack[0], StackElem::Integer(3)));
    assert!(matches!(&stack[2], StackElem::Integer(1)));
}

#[test]
fn test_swap0_nop() {
    let stack = run_and_get_stack("1 2 3 swap0");
    assert_eq!(stack.len(), 3);
    assert!(matches!(&stack[2], StackElem::Integer(3)));
}

#[test]
fn test_swap_bracket() {
    let stack = run_and_get_stack("1 2 3 4 swap(2)");
    assert!(matches!(&stack[1], StackElem::Integer(4)));
    assert!(matches!(&stack[3], StackElem::Integer(2)));
}

#[test]
fn test_drop() {
    let stack = run_and_get_stack("1 2 3 drop");
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_drop_error() {
    assert!(run_expect_error("drop"));
}

#[test]
fn test_clear() {
    let stack = run_and_get_stack("1 2 3 4 5 clear");
    assert_eq!(stack.len(), 0);
}

#[test]
fn test_roll() {
    let stack = run_and_get_stack("1 2 3 roll");
    assert!(matches!(&stack[0], StackElem::Integer(3)));
    assert!(matches!(&stack[1], StackElem::Integer(1)));
    assert!(matches!(&stack[2], StackElem::Integer(2)));
}

#[test]
fn test_top() {
    let stack = run_and_get_stack("1 2 3 top");
    assert_eq!(stack.len(), 4);
    assert!(matches!(&stack[3], StackElem::Integer(1)));
}

#[test]
fn test_dig0_nop() {
    let stack = run_and_get_stack("1 2 3 dig0");
    assert_eq!(stack.len(), 3);
    assert!(matches!(&stack[2], StackElem::Integer(3)));
}

#[test]
fn test_dig1() {
    let stack = run_and_get_stack("1 2 3 dig1");
    assert!(matches!(&stack[2], StackElem::Integer(2)));
    assert!(matches!(&stack[1], StackElem::Integer(3)));
}

#[test]
fn test_dig2() {
    let stack = run_and_get_stack("1 2 3 4 dig2");
    assert!(matches!(&stack[3], StackElem::Integer(2)));
}

#[test]
fn test_dig_bracket() {
    let stack = run_and_get_stack("1 2 3 4 dig(2)");
    assert!(matches!(&stack[3], StackElem::Integer(2)));
}

#[test]
fn test_size_empty() {
    assert!(matches!(run_and_get_top("size"), StackElem::Integer(0)));
}

#[test]
fn test_size_nonempty() {
    assert!(matches!(run_and_get_top("1 2 3 size"), StackElem::Integer(3)));
}

#[test]
fn test_empty_on_empty_stack() {
    assert!(matches!(run_and_get_top("empty"), StackElem::Boolean(true)));
}

#[test]
fn test_empty_on_nonempty_stack() {
    assert!(matches!(run_and_get_top("1 empty"), StackElem::Boolean(false)));
}

#[test]
fn test_last_on_single_element() {
    assert!(matches!(run_and_get_top("42 last"), StackElem::Boolean(true)));
}

#[test]
fn test_last_on_multiple_elements() {
    assert!(matches!(run_and_get_top("1 2 3 last"), StackElem::Boolean(false)));
}

#[test]
fn test_last_on_empty_stack() {
    assert!(matches!(run_and_get_top("last"), StackElem::Boolean(false)));
}

#[test]
fn test_nop() {
    let stack = run_and_get_stack("1 2 3 nop");
    assert_eq!(stack.len(), 3);
}

// ==================== QUOTE AND COMPOSE ====================

#[test]
fn test_quote_int() {
    let top = run_and_get_top("42 quote");
    if let StackElem::Instruction(s) = top {
        assert_eq!(s.as_ref(), "42");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_quote_bool() {
    let top = run_and_get_top("true quote");
    if let StackElem::Instruction(s) = top {
        assert_eq!(s.as_ref(), "true");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_quote_instruction() {
    let top = run_and_get_top("[2 3 +] quote");
    if let StackElem::Instruction(s) = top {
        assert_eq!(s.as_ref(), "[2 3 +]");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_compose() {
    let top = run_and_get_top("[2 +] [3 *] compose");
    if let StackElem::Instruction(s) = top {
        assert_eq!(s.as_ref(), "2 + 3 *");
    } else {
        panic!("Expected instruction");
    }
}

#[test]
fn test_compose_and_apply() {
    assert!(matches!(run_and_get_top("5 [2 +] [3 *] compose apply"), StackElem::Integer(21)));
}

#[test]
fn test_apply() {
    assert!(matches!(run_and_get_top("5 [3 +] apply"), StackElem::Integer(8)));
}

#[test]
fn test_apply_nested() {
    assert!(matches!(run_and_get_top("5 [[3 +] apply] apply"), StackElem::Integer(8)));
}

#[test]
fn test_dip() {
    let stack = run_and_get_stack("1 2 3 [+] dip");
    assert_eq!(stack.len(), 2);
    assert!(matches!(&stack[0], StackElem::Integer(3)));
    assert!(matches!(&stack[1], StackElem::Integer(3)));
}

#[test]
fn test_split_instruction() {
    let stack = run_and_get_stack("[1 2 3] split");
    assert_eq!(stack.len(), 3);
}

#[test]
fn test_split_string() {
    let stack = run_and_get_stack("\"hello world\" split");
    assert_eq!(stack.len(), 2);
}

// ==================== CONTROL FLOW ====================

#[test]
fn test_if_true() {
    assert!(matches!(run_and_get_top("true [42] [0] if"), StackElem::Integer(42)));
}

#[test]
fn test_if_false() {
    assert!(matches!(run_and_get_top("false [42] [0] if"), StackElem::Integer(0)));
}

#[test]
fn test_if_with_condition() {
    assert!(matches!(run_and_get_top("5 [10] [20] if(3 <)"), StackElem::Integer(20)));
}

#[test]
fn test_if_with_condition_true() {
    assert!(matches!(run_and_get_top("2 [10] [20] if(3 <)"), StackElem::Integer(10)));
}

#[test]
fn test_loop_while_do() {
    assert!(matches!(run_and_get_top("1 [1 + dup 5 <] loop"), StackElem::Integer(5)));
}

#[test]
fn test_loop_with_condition() {
    assert!(matches!(run_and_get_top("0 [1 +] loop(dup 5 <)"), StackElem::Integer(5)));
}

#[test]
fn test_loop_no_iterations() {
    assert!(matches!(run_and_get_top("10 [1 +] loop(dup 5 <)"), StackElem::Integer(10)));
}

#[test]
fn test_times() {
    assert!(matches!(run_and_get_top("0 [1 +] times(5)"), StackElem::Integer(5)));
}

#[test]
fn test_times_zero() {
    assert!(matches!(run_and_get_top("42 [1 +] times(0)"), StackElem::Integer(42)));
}

#[test]
fn test_try_success() {
    assert!(matches!(run_and_get_top("[1 2 +] try"), StackElem::Boolean(true)));
}

#[test]
fn test_try_failure() {
    assert!(matches!(run_and_get_top("[drop] try"), StackElem::Boolean(false)));
}

#[test]
fn test_try_preserves_stack_on_success() {
    let stack = run_and_get_stack("1 2 [+] try");
    assert_eq!(stack.len(), 2);
    assert!(matches!(&stack[0], StackElem::Integer(3)));
    assert!(matches!(&stack[1], StackElem::Boolean(true)));
}

// ==================== DEFINE, DELETE, ISDEF ====================

#[test]
fn test_define_simple() {
    assert!(matches!(
        run_and_get_top("[2 *] define(double) 21 double"),
        StackElem::Integer(42)
    ));
}

#[test]
fn test_define_recursive() {
    let top = run_and_get_top(
        "[dup 1 <= [drop 1] [dup 1 - fact *] if] define(fact) 5 fact"
    );
    assert!(matches!(top, StackElem::Integer(120)));
}

#[test]
fn test_define_with_bracket_ops() {
    let stack = run_and_get_stack("[swap2 swap drop swap] define(nip) 1 2 3 nip");
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_isdef_true() {
    assert!(matches!(
        run_and_get_top("[nop] define(test) isdef(test)"),
        StackElem::Boolean(true)
    ));
}

#[test]
fn test_isdef_false() {
    assert!(matches!(
        run_and_get_top("isdef(nonexistent)"),
        StackElem::Boolean(false)
    ));
}

#[test]
fn test_delete() {
    assert!(matches!(
        run_and_get_top("[nop] define(test) delete(test) isdef(test)"),
        StackElem::Boolean(false)
    ));
}

#[test]
fn test_delete_and_redefine() {
    let top = run_and_get_top(
        "[2 *] define(f) delete(f) [3 *] define(f) 10 f"
    );
    assert!(matches!(top, StackElem::Integer(30)));
}

// ==================== TYPE OPERATIONS ====================

#[test]
fn test_type_int() {
    let stack = run_and_get_stack("42 type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "INT");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_float() {
    let stack = run_and_get_stack("3.14 type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "FLOAT");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_bool() {
    let stack = run_and_get_stack("true type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "BOOL");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_string() {
    let stack = run_and_get_stack("\"hello\" type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "STR");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_instruction() {
    let stack = run_and_get_stack("[nop] type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "INSTR");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_none() {
    let stack = run_and_get_stack("none type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "NONE");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_stack() {
    let stack = run_and_get_stack("stack type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "STACK");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_type() {
    let stack = run_and_get_stack("INT type");
    if let StackElem::Type(t) = &stack[1] {
        assert_eq!(t.as_str(), "TYPE");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_constant_int() {
    if let StackElem::Type(t) = run_and_get_top("INT") {
        assert_eq!(t.as_str(), "INT");
    } else {
        panic!("Expected type");
    }
}

#[test]
fn test_type_comparison() {
    assert!(matches!(run_and_get_top("42 type INT =="), StackElem::Boolean(true)));
}

#[test]
fn test_type_comparison_false() {
    assert!(matches!(run_and_get_top("42 type FLOAT =="), StackElem::Boolean(false)));
}

// ==================== INNER STACK OPERATIONS ====================

#[test]
fn test_stack_create() {
    let stack = run_and_get_stack("stack");
    assert_eq!(stack.len(), 1);
    assert!(matches!(&stack[0], StackElem::InnerStack(_)));
}

#[test]
fn test_stack_push() {
    let stack = run_and_get_stack("stack 1 push 2 push 3 push");
    assert_eq!(stack.len(), 1);
    if let StackElem::InnerStack(s) = &stack[0] {
        assert_eq!(s.borrow().len(), 3);
    } else {
        panic!("Expected inner stack");
    }
}

#[test]
fn test_stack_pop() {
    let stack = run_and_get_stack("stack 1 push 2 push pop");
    assert_eq!(stack.len(), 2);
    assert!(matches!(&stack[1], StackElem::Integer(2)));
}

#[test]
fn test_stack_pop_empty() {
    let stack = run_and_get_stack("stack pop");
    assert_eq!(stack.len(), 2);
    assert!(matches!(&stack[1], StackElem::None));
}

#[test]
fn test_stack_literal() {
    let stack = run_and_get_stack("{1 2 3}");
    assert_eq!(stack.len(), 1);
    if let StackElem::InnerStack(s) = &stack[0] {
        assert_eq!(s.borrow().len(), 3);
    } else {
        panic!("Expected inner stack");
    }
}

#[test]
fn test_stack_literal_empty() {
    let stack = run_and_get_stack("{}");
    assert_eq!(stack.len(), 1);
    if let StackElem::InnerStack(s) = &stack[0] {
        assert_eq!(s.borrow().len(), 0);
    } else {
        panic!("Expected inner stack");
    }
}

#[test]
fn test_compress() {
    let stack = run_and_get_stack("1 2 3 compress");
    assert_eq!(stack.len(), 1);
    if let StackElem::InnerStack(s) = &stack[0] {
        assert_eq!(s.borrow().len(), 3);
    } else {
        panic!("Expected inner stack");
    }
}

#[test]
fn test_compress_empty() {
    let stack = run_and_get_stack("compress");
    assert_eq!(stack.len(), 1);
    if let StackElem::InnerStack(s) = &stack[0] {
        assert_eq!(s.borrow().len(), 0);
    } else {
        panic!("Expected inner stack");
    }
}

#[test]
fn test_inject() {
    let stack = run_and_get_stack("stack 1 push 2 push [+] inject");
    assert_eq!(stack.len(), 1);
    if let StackElem::InnerStack(s) = &stack[0] {
        let inner = s.borrow();
        assert_eq!(inner.len(), 1);
        assert!(matches!(&inner.content[0], StackElem::Integer(3)));
    } else {
        panic!("Expected inner stack");
    }
}

#[test]
fn test_inject_n() {
    let stack = run_and_get_stack("{1 2} {3 4} [+] inject2");
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_split_inner_stack() {
    let stack = run_and_get_stack("{1 2 3} split");
    assert_eq!(stack.len(), 3);
}

#[test]
fn test_pinject2() {
    let stack = run_and_get_stack("{1 2} {3 4} [+] pinject2");
    assert_eq!(stack.len(), 2);
    
    if let StackElem::InnerStack(s) = &stack[0] {
        let inner = s.borrow();
        assert_eq!(inner.len(), 1);
        assert!(matches!(&inner.content[0], StackElem::Integer(3)));
    } else {
        panic!("Expected inner stack");
    }
    
    if let StackElem::InnerStack(s) = &stack[1] {
        let inner = s.borrow();
        assert_eq!(inner.len(), 1);
        assert!(matches!(&inner.content[0], StackElem::Integer(7)));
    } else {
        panic!("Expected inner stack");
    }
}

#[test]
fn test_pinject3() {
    let stack = run_and_get_stack("{10} {20} {30} [2 *] pinject3");
    assert_eq!(stack.len(), 3);
    
    if let StackElem::InnerStack(s) = &stack[0] {
        assert!(matches!(&s.borrow().content[0], StackElem::Integer(20)));
    }
    if let StackElem::InnerStack(s) = &stack[1] {
        assert!(matches!(&s.borrow().content[0], StackElem::Integer(40)));
    }
    if let StackElem::InnerStack(s) = &stack[2] {
        assert!(matches!(&s.borrow().content[0], StackElem::Integer(60)));
    }
}

#[test]
fn test_pinject_complex() {
    let stack = run_and_get_stack("{1 2 3} {4 5 6} {7 8 9} [[+] loop(size 1 >)] pinject3");
    assert_eq!(stack.len(), 3);
    
    if let StackElem::InnerStack(s) = &stack[0] {
        assert!(matches!(&s.borrow().content[0], StackElem::Integer(6)));
    }
    if let StackElem::InnerStack(s) = &stack[1] {
        assert!(matches!(&s.borrow().content[0], StackElem::Integer(15)));
    }
    if let StackElem::InnerStack(s) = &stack[2] {
        assert!(matches!(&s.borrow().content[0], StackElem::Integer(24)));
    }
}

// ==================== I/O OPERATIONS ====================

#[test]
fn test_print_preserves_stack() {
    let stack = run_and_get_stack("42 print");
    assert_eq!(stack.len(), 1);
}

#[test]
fn test_printall_preserves_stack() {
    let stack = run_and_get_stack("1 2 3 printall");
    assert_eq!(stack.len(), 3);
}

// ==================== COMPLEX EXAMPLES ====================

#[test]
fn test_fibonacci() {
    let top = run_and_get_top(
        "[dup dup 1 == swap 0 == or not [dup 1 - fib swap 2 - fib +] [nop] if] define(fib) 10 fib"
    );
    assert!(matches!(top, StackElem::Integer(55)));
}

#[test]
fn test_sum_1_to_10() {
    let top = run_and_get_top("0 1 [dup swap2 + swap 1 + dup 10 <=] loop drop");
    assert!(matches!(top, StackElem::Integer(55)));
}

#[test]
fn test_gcd() {
    let top = run_and_get_top(
        "[dup 0 == [drop] [dup swap2 swap % gcd] if] define(gcd) 48 18 gcd"
    );
    assert!(matches!(top, StackElem::Integer(6)));
}

#[test]
fn test_nested_if() {
    let top = run_and_get_top("true [true [1] [2] if] [3] if");
    assert!(matches!(top, StackElem::Integer(1)));
}

#[test]
fn test_double_loop() {
    let top = run_and_get_top("0 [1 +] times(3) 0 [1 +] times(4) *");
    assert!(matches!(top, StackElem::Integer(12)));
}

#[test]
fn test_higher_order_dip() {
    let stack = run_and_get_stack("[2 *] define(double) 1 2 3 [double] dip [double] dip double");
    assert_eq!(stack.len(), 3);
    assert!(matches!(&stack[0], StackElem::Integer(1)));
    assert!(matches!(&stack[1], StackElem::Integer(8)));
    assert!(matches!(&stack[2], StackElem::Integer(6)));
}

#[test]
fn test_quote_compose_apply() {
    let top = run_and_get_top("5 quote [1 +] compose apply");
    assert!(matches!(top, StackElem::Integer(6)));
}

#[test]
fn test_complex_define_with_conditionals() {
    let top = run_and_get_top("[dup 0 < [--] [nop] if] define(abs) -42 abs");
    assert!(matches!(top, StackElem::Integer(42)));
}

#[test]
fn test_complex_define_with_loop() {
    let top = run_and_get_top(
        "[1 swap [dup swap2 * swap 1 - dup 0 >] loop drop] define(myfact) 5 myfact"
    );
    assert!(matches!(top, StackElem::Integer(120)));
}

// ==================== ERROR HANDLING ====================

#[test]
fn test_stack_underflow() {
    assert!(run_expect_error("drop"));
}

#[test]
fn test_invalid_operands_add() {
    assert!(run_expect_error("true 5 +"));
}

#[test]
fn test_invalid_operands_and() {
    assert!(run_expect_error("1 2 and"));
}

#[test]
fn test_undefined_instruction() {
    assert!(run_expect_error("nonexistent_instruction"));
}

#[test]
fn test_invalid_if_operands() {
    assert!(run_expect_error("5 [1] [2] if"));
}

// ==================== EDGE CASES ====================

#[test]
fn test_large_number() {
    assert!(matches!(
        run_and_get_top("9223372036854775807"),
        StackElem::Integer(9223372036854775807)
    ));
}

#[test]
fn test_deeply_nested_quotes() {
    let top = run_and_get_top("[[[1]]]");
    assert!(matches!(top, StackElem::Instruction(_)));
}

#[test]
fn test_empty_quote() {
    let top = run_and_get_top("[]");
    assert!(matches!(top, StackElem::Instruction(_)));
}

#[test]
fn test_apply_empty_quote() {
    let stack = run_and_get_stack("1 2 [] apply");
    assert_eq!(stack.len(), 2);
}

#[test]
fn test_many_operations() {
    let top = run_and_get_top("1 2 + 3 * 4 - 5 /");
    if let StackElem::Floating(f) = top {
        assert!((f - 1.0).abs() < 0.001);
    } else {
        panic!("Expected float");
    }
}

#[test]
fn test_swap_with_different_types() {
    let stack = run_and_get_stack("42 \"hello\" swap");
    assert!(matches!(&stack[0], StackElem::String(_)));
    assert!(matches!(&stack[1], StackElem::Integer(42)));
}

#[test]
fn test_dup_instruction() {
    let stack = run_and_get_stack("[1 2 +] dup");
    assert_eq!(stack.len(), 2);
    assert!(matches!(&stack[0], StackElem::Instruction(_)));
    assert!(matches!(&stack[1], StackElem::Instruction(_)));
}
