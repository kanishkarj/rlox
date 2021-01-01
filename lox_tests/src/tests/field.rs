
use rlox_core::frontend::parser::Parser;
use rlox_core::frontend::lexer::*;
use std::fs::read_to_string;
use std::io::{self, stdin, stdout, Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use rlox_core::runtime::interpreter::{Interpreter};
use rlox_core::frontend::resolver::Resolver;
// use rlox_core::runtime::system_calls::SystemInterfaceMock;
// use logos::{source::Source, Logos};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
// use rlox_core::runtime::definitions::object::Object;
use rlox_core::error::LoxError;
use super::*;

                test_succeed!(
                    call_function_field,
                    ".././test-scripts/field/call_function_field.lox",
                    "bar",1,2
                );

                
            test_fail!(
                call_nonfunction_field,
                ".././test-scripts/field/call_nonfunction_field.lox",
                LoxError::RuntimeError(String::from("not fn"), 6, String::from(""))
            );

            
                test_succeed!(
                    get_and_set_method,
                    ".././test-scripts/field/get_and_set_method.lox",
                    "other",1,"method",2
                );

                
            test_fail!(
                get_on_bool,
                ".././test-scripts/field/get_on_bool.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
            test_fail!(
                get_on_class,
                ".././test-scripts/field/get_on_class.lox",
                LoxError::RuntimeError(String::from("bar"), 2, String::from(""))
            );

            
            test_fail!(
                get_on_function,
                ".././test-scripts/field/get_on_function.lox",
                LoxError::RuntimeError(String::from("bar"), 3, String::from(""))
            );

            
            test_fail!(
                get_on_nil,
                ".././test-scripts/field/get_on_nil.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
            test_fail!(
                get_on_num,
                ".././test-scripts/field/get_on_num.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
            test_fail!(
                get_on_string,
                ".././test-scripts/field/get_on_string.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
                test_succeed!(
                    many,
                    ".././test-scripts/field/many.lox",
                    "apple",
                    "apricot",
                    "avocado",
                    "banana",
                    "bilberry",
                    "blackberry",
                    "blackcurrant",
                    "blueberry",
                    "boysenberry",
                    "cantaloupe",
                    "cherimoya",
                    "cherry",
                    "clementine",
                    "cloudberry",
                    "coconut",
                    "cranberry",
                    "currant",
                    "damson",
                    "date",
                    "dragonfruit",
                    "durian",
                    "elderberry",
                    "feijoa",
                    "fig",
                    "gooseberry",
                    "grape",
                    "grapefruit",
                    "guava",
                    "honeydew",
                    "huckleberry",
                    "jabuticaba",
                    "jackfruit",
                    "jambul",
                    "jujube",
                    "juniper",
                    "kiwifruit",
                    "kumquat",
                    "lemon",
                    "lime",
                    "longan",
                    "loquat",
                    "lychee",
                    "mandarine",
                    "mango",
                    "marionberry",
                    "melon",
                    "miracle",
                    "mulberry",
                    "nance",
                    "nectarine",
                    "olive",
                    "orange",
                    "papaya",
                    "passionfruit",
                    "peach",
                    "pear",
                    "persimmon",
                    "physalis",
                    "pineapple",
                    "plantain",
                    "plum",
                    "plumcot",
                    "pomegranate",
                    "pomelo",
                    "quince",
                    "raisin",
                    "rambutan",
                    "raspberry",
                    "redcurrant",
                    "salak",
                    "salmonberry",
                    "satsuma",
                    "strawberry",
                    "tamarillo",
                    "tamarind",
                    "tangerine",
                    "tomato",
                    "watermelon",
                    "yuzu"
                );

                
                test_succeed!(
                    method,
                    ".././test-scripts/field/method.lox",
                    "got method","arg"
                );

                
                test_succeed!(
                    method_binds_this,
                    ".././test-scripts/field/method_binds_this.lox",
                    "foo1",1
                );

                
                test_succeed!(
                    on_instance,
                    ".././test-scripts/field/on_instance.lox",
                    "bar value","baz value","bar value","baz value"
                );

                
            test_fail!(
                set_evaluation_order,
                ".././test-scripts/field/set_evaluation_order.lox",
                LoxError::SemanticError(String::from("undefined1"), 1, String::from(""))
            );

            
            test_fail!(
                set_on_bool,
                ".././test-scripts/field/set_on_bool.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
            test_fail!(
                set_on_class,
                ".././test-scripts/field/set_on_class.lox",
                LoxError::RuntimeError(String::from("bar"), 2, String::from(""))
            );

            
            test_fail!(
                set_on_function,
                ".././test-scripts/field/set_on_function.lox",
                LoxError::RuntimeError(String::from("bar"), 3, String::from(""))
            );

            
            test_fail!(
                set_on_nil,
                ".././test-scripts/field/set_on_nil.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
            test_fail!(
                set_on_num,
                ".././test-scripts/field/set_on_num.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
            test_fail!(
                set_on_string,
                ".././test-scripts/field/set_on_string.lox",
                LoxError::RuntimeError(String::from("foo"), 1, String::from(""))
            );

            
            test_fail!(
                undefined,
                ".././test-scripts/field/undefined.lox",
                LoxError::RuntimeError(String::from("bar"), 4, String::from(""))
            );

            