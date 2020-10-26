#!/usr/bin/python3

import sys
import glob
import re 

dir_name = sys.argv[1]
files = glob.glob(dir_name + "/*.lox")
out_filename =  dir_name.split("/")[-1] if not dir_name.split("/")[-1] == "" else dir_name.split("/")[-2]
out_filename =  "rlox_core/src/tests/" + out_filename + ".rs"

# print(files, out_filename)
#  arr = os.listdir(dir_name)
# print(arr)

out_string = """
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
"""

for test_file in files:
    with open(str(test_file), 'r') as file:  # Use file to refer to the file object
        file_str = file.read()
        test_name = file.name.split("/")[-1].split(".")[0]
        if file_str.find("Error") >= 0 or file_str.find("error") >= 0 :
            out_string += """
            test_fail!(
                {test_name},
                "../{test_file}",
                LoxError::RuntimeError(String::from(""), 0, String::from(""))
            );\n
            """.format(test_name=test_name, test_file=test_file)
        else:
            tests = re.findall("expect: [a-zA-Z_0-9 <>]*", file_str)
            tests = [x.split(": ")[1] for x in tests]
            if len(tests) > 0 :
                out_string += """
                test_succeed!(
                    {test_name},
                    "../{test_file}",
                    {test_values}
                );\n
                """.format(test_name=test_name, test_values=",".join(tests), test_file=test_file)
        



with open(str(out_filename), "w") as file:
    file.write(out_string)