use super::*;
use crate::error::LoxError;
use crate::frontend::lexer::*;
use crate::frontend::parser::Parser;
use crate::frontend::resolver::Resolver;
use crate::runtime::definitions::lox_class::{LoxClass, LoxInstance};
use crate::runtime::definitions::object::Object;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::system_calls::SystemInterfaceMock;
use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};

test_succeed!(
    empty,
    "../test-scripts/class/empty.lox",
    LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None)
);

test_fail!(
    inherit_self,
    "../test-scripts/class/inherit_self.lox",
    LoxError::SemanticError(String::from("Foo"), 1, String::from(""))
);

test_succeed!(
    inherited_method,
    "../test-scripts/class/inherited_method.lox",
    "in foo",
    "in bar",
    "in baz"
);

test_succeed!(
    local_inherit_other,
    "../test-scripts/class/local_inherit_other.lox",
    LoxClass::new(String::from("B"), Rc::new(HashMap::new()), None)
);

test_fail!(
    local_inherit_self,
    "../test-scripts/class/local_inherit_self.lox",
    LoxError::SemanticError(String::from("Foo"), 2, String::from(""))
);

test_succeed!(
    local_reference_self,
    "../test-scripts/class/local_reference_self.lox",
    LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None)
);

test_succeed!(
    reference_self,
    "../test-scripts/class/reference_self.lox",
    LoxClass::new(String::from("Foo"), Rc::new(HashMap::new()), None)
);
