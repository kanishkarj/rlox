use crate::frontend::parser::Parser;
use crate::frontend::lexer::*;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::runtime::interpreter::{Interpreter};
use crate::frontend::resolver::Resolver;
use crate::runtime::system_calls::SystemInterfaceMock;
use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::definitions::object::Object;
use crate::error::LoxError;
use super::*;

test_succeed!(
    assign_to_closure,
    "../test-scripts/closure/assign_to_closure.lox",
    "local",
"after f",
"after f",
"after g"
);


test_succeed!(
    assign_to_shadowed_later,
    "../test-scripts/closure/assign_to_shadowed_later.lox",
    "inner",
"assigned"
);

test_succeed!(
    close_over_function_parameter,
    "../test-scripts/closure/close_over_function_parameter.lox",
    "param"
);
test_succeed!(
    close_over_later_variable,
    "../test-scripts/closure/close_over_later_variable.lox",
    "b",
    "a"
);
test_succeed!(
    close_over_method_parameter,
    "../test-scripts/closure/close_over_method_parameter.lox",
    "param"
);
test_succeed!(
    closed_closure_in_function,
    "../test-scripts/closure/closed_closure_in_function.lox",
    "local"
);
test_succeed!(
    nested_closure,
    "../test-scripts/closure/nested_closure.lox",
    "a",
    "b",
    "c"
);
test_succeed!(
    open_closure_in_function,
    "../test-scripts/closure/open_closure_in_function.lox",
    "local"
);
test_succeed!(
    reference_closure_multiple_times,
    "../test-scripts/closure/reference_closure_multiple_times.lox",
    "a",
    "a"
);
test_succeed!(
    reuse_closure_slot,
    "../test-scripts/closure/reuse_closure_slot.lox",
    "a"
);
test_succeed!(
    shadow_closure_with_local,
    "../test-scripts/closure/shadow_closure_with_local.lox",
    "closure",
    "shadow",
    "closure"
);
test_succeed!(
    unused_closure,
    "../test-scripts/closure/unused_closure.lox",
    "ok"
);
test_succeed!(
    unused_later_closure,
    "../test-scripts/closure/unused_later_closure.lox",
    "a"
);