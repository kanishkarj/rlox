use rlox_core::frontend::lexer::*;
use rlox_core::frontend::parser::Parser;
use rlox_core::frontend::resolver::Resolver;
use rlox_core::runtime::interpreter::Interpreter;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
// use rlox_core::runtime::system_calls::SystemInterfaceMock;
// use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
// use rlox_core::runtime::definitions::object::Object;
use super::*;
use rlox_core::error::LoxError;

test_succeed!(
    bound_method,
    ".././test-scripts/super/bound_method.lox",
    "A.method(arg)"
);

test_succeed!(
    call_other_method,
    ".././test-scripts/super/call_other_method.lox",
    "Derived.bar()",
    "Base.foo()"
);

test_succeed!(
    call_same_method,
    ".././test-scripts/super/call_same_method.lox",
    "Derived.foo()",
    "Base.foo()"
);

test_succeed!(closure, ".././test-scripts/super/closure.lox", "Base");

test_succeed!(
    constructor,
    ".././test-scripts/super/constructor.lox",
    "Derived.init()",
    "Base.init(a, b)"
);

test_fail!(
    extra_arguments,
    ".././test-scripts/super/extra_arguments.lox",
    LoxError::RuntimeError(String::from("foo"), 10, String::from(""))
);

test_succeed!(
    indirectly_inherited,
    ".././test-scripts/super/indirectly_inherited.lox",
    "C.foo()",
    "A.foo()"
);

test_fail!(
    missing_arguments,
    ".././test-scripts/super/missing_arguments.lox",
    LoxError::RuntimeError(String::from("foo"), 9, String::from(""))
);

test_fail!(
    no_superclass_bind,
    ".././test-scripts/super/no_superclass_bind.lox",
    LoxError::SemanticError(String::from("super"), 3, String::from(""))
);

test_fail!(
    no_superclass_call,
    ".././test-scripts/super/no_superclass_call.lox",
    LoxError::SemanticError(String::from("super"), 3, String::from(""))
);

test_fail!(
    no_superclass_method,
    ".././test-scripts/super/no_superclass_method.lox",
    LoxError::RuntimeError(String::from("super"), 5, String::from(""))
);

test_fail!(
    parenthesized,
    ".././test-scripts/super/parenthesized.lox",
    LoxError::ParserError(String::from(")"), 8, String::from(""))
);

test_succeed!(
    reassign_superclass,
    ".././test-scripts/super/reassign_superclass.lox",
    "Base.method()",
    "Base.method()"
);

test_fail!(
    super_at_top_level,
    ".././test-scripts/super/super_at_top_level.lox",
    LoxError::SemanticError(String::from("super"), 1, String::from(""))
);

test_succeed!(
    super_in_closure_in_inherited_method,
    ".././test-scripts/super/super_in_closure_in_inherited_method.lox",
    "A"
);

test_succeed!(
    super_in_inherited_method,
    ".././test-scripts/super/super_in_inherited_method.lox",
    "A"
);

test_fail!(
    super_in_top_level_function,
    ".././test-scripts/super/super_in_top_level_function.lox",
    LoxError::SemanticError(String::from("super"), 1, String::from(""))
);

test_fail!(
    super_without_dot,
    ".././test-scripts/super/super_without_dot.lox",
    LoxError::ParserError(String::from(";"), 6, String::from(""))
);

test_fail!(
    super_without_name,
    ".././test-scripts/super/super_without_name.lox",
    LoxError::ParserError(String::from(";"), 5, String::from(""))
);

test_succeed!(
    this_in_superclass_method,
    ".././test-scripts/super/this_in_superclass_method.lox",
    "a",
    "b"
);
