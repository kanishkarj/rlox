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

test_fail!(
    collide_with_parameter,
    ".././test-scripts/variable/collide_with_parameter.lox",
    LoxError::RuntimeError(String::from("a"), 2, String::from(""))
);

test_fail!(
    duplicate_local,
    ".././test-scripts/variable/duplicate_local.lox",
    LoxError::RuntimeError(String::from("a"), 3, String::from(""))
);

test_fail!(
    duplicate_parameter,
    ".././test-scripts/variable/duplicate_parameter.lox",
    LoxError::RuntimeError(String::from("arg"), 2, String::from(""))
);

test_succeed!(
    early_bound,
    ".././test-scripts/variable/early_bound.lox",
    "outer",
    "outer"
);

test_succeed!(
    in_middle_of_block,
    ".././test-scripts/variable/in_middle_of_block.lox",
    "a",
    "a b",
    "a c",
    "a b d"
);

test_succeed!(
    in_nested_block,
    ".././test-scripts/variable/in_nested_block.lox",
    "outer"
);

test_succeed!(
    local_from_method,
    ".././test-scripts/variable/local_from_method.lox",
    "variable"
);

test_succeed!(
    redeclare_global,
    ".././test-scripts/variable/redeclare_global.lox",
    Object::Nil
);

test_succeed!(
    redefine_global,
    ".././test-scripts/variable/redefine_global.lox",
    "2"
);

test_succeed!(
    scope_reuse_in_different_blocks,
    ".././test-scripts/variable/scope_reuse_in_different_blocks.lox",
    "first",
    "second"
);

test_succeed!(
    shadow_and_local,
    ".././test-scripts/variable/shadow_and_local.lox",
    "outer",
    "inner"
);

test_succeed!(
    shadow_global,
    ".././test-scripts/variable/shadow_global.lox",
    "shadow",
    "global"
);

test_succeed!(
    shadow_local,
    ".././test-scripts/variable/shadow_local.lox",
    "shadow",
    "local"
);

test_fail!(
    undefined_global,
    ".././test-scripts/variable/undefined_global.lox",
    LoxError::RuntimeError(String::from("notDefined"), 1, String::from(""))
);

test_fail!(
    undefined_local,
    ".././test-scripts/variable/undefined_local.lox",
    LoxError::RuntimeError(String::from("notDefined"), 2, String::from(""))
);

test_succeed!(
    uninitialized,
    ".././test-scripts/variable/uninitialized.lox",
    Object::Nil
);

test_succeed!(
    unreached_undefined,
    ".././test-scripts/variable/unreached_undefined.lox",
    "ok"
);

test_fail!(
    use_false_as_var,
    ".././test-scripts/variable/use_false_as_var.lox",
    LoxError::ParserError(String::from("false"), 2, String::from(""))
);

test_succeed!(
    use_global_in_initializer,
    ".././test-scripts/variable/use_global_in_initializer.lox",
    "value"
);

test_fail!(
    use_local_in_initializer,
    ".././test-scripts/variable/use_local_in_initializer.lox",
    LoxError::SemanticError(String::from("a"), 3, String::from(""))
);

test_fail!(
    use_nil_as_var,
    ".././test-scripts/variable/use_nil_as_var.lox",
    LoxError::ParserError(String::from("nil"), 2, String::from(""))
);

test_fail!(
    use_this_as_var,
    ".././test-scripts/variable/use_this_as_var.lox",
    LoxError::ParserError(String::from("this"), 2, String::from(""))
);
