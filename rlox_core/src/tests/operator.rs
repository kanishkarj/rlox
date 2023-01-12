use super::*;
use crate::error::LoxError;
use crate::frontend::lexer::*;
use crate::frontend::parser::Parser;
use crate::frontend::resolver::Resolver;
use crate::runtime::definitions::object::Object;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::system_calls::SystemInterfaceMock;
use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

test_succeed!(add, ".././test-scripts/operator/add.lox", 579, "string");

test_fail!(
    add_bool_nil,
    ".././test-scripts/operator/add_bool_nil.lox",
    LoxError::RuntimeError(String::from("+"), 1, String::from(""))
);

test_fail!(
    add_bool_num,
    ".././test-scripts/operator/add_bool_num.lox",
    LoxError::RuntimeError(String::from("+"), 1, String::from(""))
);

test_fail!(
    add_bool_string,
    ".././test-scripts/operator/add_bool_string.lox",
    LoxError::RuntimeError(String::from("+"), 1, String::from(""))
);

test_fail!(
    add_nil_nil,
    ".././test-scripts/operator/add_nil_nil.lox",
    LoxError::RuntimeError(String::from("+"), 1, String::from(""))
);

test_fail!(
    add_num_nil,
    ".././test-scripts/operator/add_num_nil.lox",
    LoxError::RuntimeError(String::from("+"), 1, String::from(""))
);

test_fail!(
    add_string_nil,
    ".././test-scripts/operator/add_string_nil.lox",
    LoxError::RuntimeError(String::from("+"), 1, String::from(""))
);

test_succeed!(
    comparison,
    ".././test-scripts/operator/comparison.lox",
    true,
    false,
    false,
    true,
    true,
    false,
    false,
    false,
    true,
    false,
    true,
    true,
    false,
    false,
    false,
    false,
    true,
    true,
    true,
    true
);

test_succeed!(divide, ".././test-scripts/operator/divide.lox", 4, 1);

test_fail!(
    divide_nonnum_num,
    ".././test-scripts/operator/divide_nonnum_num.lox",
    LoxError::RuntimeError(String::from("/"), 1, String::from(""))
);

test_fail!(
    divide_num_nonnum,
    ".././test-scripts/operator/divide_num_nonnum.lox",
    LoxError::RuntimeError(String::from("/"), 1, String::from(""))
);

test_succeed!(
    equals,
    ".././test-scripts/operator/equals.lox",
    true,
    true,
    false,
    true,
    false,
    true,
    false,
    false,
    false,
    false
);

//TODO: handle equality on other types in object
// test_succeed!(
//     equals_class,
//     ".././test-scripts/operator/equals_class.lox",
//     true,false,false,true,false,false,false,false
// );

// test_succeed!(
//     equals_method,
//     ".././test-scripts/operator/equals_method.lox",
//     true,false
// );

test_fail!(
    greater_nonnum_num,
    ".././test-scripts/operator/greater_nonnum_num.lox",
    LoxError::RuntimeError(String::from(">"), 1, String::from(""))
);

test_fail!(
    greater_num_nonnum,
    ".././test-scripts/operator/greater_num_nonnum.lox",
    LoxError::RuntimeError(String::from(">"), 1, String::from(""))
);

test_fail!(
    greater_or_equal_nonnum_num,
    ".././test-scripts/operator/greater_or_equal_nonnum_num.lox",
    LoxError::RuntimeError(String::from(">="), 1, String::from(""))
);

test_fail!(
    greater_or_equal_num_nonnum,
    ".././test-scripts/operator/greater_or_equal_num_nonnum.lox",
    LoxError::RuntimeError(String::from(">="), 1, String::from(""))
);

test_fail!(
    less_nonnum_num,
    ".././test-scripts/operator/less_nonnum_num.lox",
    LoxError::RuntimeError(String::from("<"), 1, String::from(""))
);

test_fail!(
    less_num_nonnum,
    ".././test-scripts/operator/less_num_nonnum.lox",
    LoxError::RuntimeError(String::from("<"), 1, String::from(""))
);

test_fail!(
    less_or_equal_nonnum_num,
    ".././test-scripts/operator/less_or_equal_nonnum_num.lox",
    LoxError::RuntimeError(String::from("<="), 1, String::from(""))
);

test_fail!(
    less_or_equal_num_nonnum,
    ".././test-scripts/operator/less_or_equal_num_nonnum.lox",
    LoxError::RuntimeError(String::from("<="), 1, String::from(""))
);

test_succeed!(
    multiply,
    ".././test-scripts/operator/multiply.lox",
    15,
    3.702
);

test_fail!(
    multiply_nonnum_num,
    ".././test-scripts/operator/multiply_nonnum_num.lox",
    LoxError::RuntimeError(String::from("*"), 1, String::from(""))
);

test_fail!(
    multiply_num_nonnum,
    ".././test-scripts/operator/multiply_num_nonnum.lox",
    LoxError::RuntimeError(String::from("*"), 1, String::from(""))
);

test_succeed!(negate, ".././test-scripts/operator/negate.lox", -3, 3, -3);

test_fail!(
    negate_nonnum,
    ".././test-scripts/operator/negate_nonnum.lox",
    LoxError::RuntimeError(String::from("-"), 1, String::from(""))
);

test_succeed!(
    not,
    ".././test-scripts/operator/not.lox",
    false,
    true,
    true,
    false,
    false,
    true,
    false,
    false
);

test_succeed!(
    not_class,
    ".././test-scripts/operator/not_class.lox",
    false,
    false
);

test_succeed!(
    not_equals,
    ".././test-scripts/operator/not_equals.lox",
    false,
    false,
    true,
    false,
    true,
    false,
    true,
    true,
    true,
    true
);

test_succeed!(subtract, ".././test-scripts/operator/subtract.lox", 1, 0);

test_fail!(
    subtract_nonnum_num,
    ".././test-scripts/operator/subtract_nonnum_num.lox",
    LoxError::RuntimeError(String::from("-"), 1, String::from(""))
);

test_fail!(
    subtract_num_nonnum,
    ".././test-scripts/operator/subtract_num_nonnum.lox",
    LoxError::RuntimeError(String::from("-"), 1, String::from(""))
);
