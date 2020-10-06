#!/usr/bin/python3

import sys

def defineStruct(structName: str, types: dict, f):
    f.write('#[derive(Debug, Clone)] \npub struct {structName} {{ \n'.format(structName=structName.strip()))
    for typeStr in types[structName]:
        (typeS, fieldS) = str(typeStr).strip().split(":")
        f.write("   pub {fieldS}: {typeS}, \n".format(typeS=typeS, fieldS=fieldS))
    f.write("} \n\n")

def defineStructMethods(structName: str, types: dict, f):
    f.write('impl {structName} {{ \n'.format(structName=structName))
    paramString = ""
    initString = ""
    for typeStr in types[structName]:
        (typeS, fieldS) = str(typeStr).strip().split(":")
        paramString += fieldS + ": " + typeS + ","
        initString += fieldS + ",\n"
    f.write(
        """
        pub fn new({paramString}) -> Self {{
            Self {{
                {initString}
            }}
        }}

        """.format(paramString=paramString, initString=initString, structName=structName)
        # pub fn accept<R>(&mut self, vis: Box<dyn Visitor<R>>) -> R {{
        #     vis.visit{structName}Expr(&mut self)
        # }}
    )
    f.write("} \n\n")

def defineExpr(f, baseName, types: list):

    f.write('#[derive(Debug, Clone)] \npub enum {baseName} {{ \n'.format(baseName=baseName))
    for typeStr in types:
        f.write("   {typeStr}(Box<{typeStr}>), \n".format(typeStr=typeStr))
    
    if baseName == "Expr":
        f.write("   Literal(Literal), \n")

    f.write("} \n\n")

def defineAst(outputDir: str, baseName: str, types: dict):
    filename = outputDir + '/' + baseName + '.rs'
    with open(filename, 'w', encoding = 'utf-8') as f:
        f.write("use crate::scanner::*; \n")
        f.write("use crate::grammar::LoxCallable; \n")
        
        if baseName != "Expr":
            f.write("use crate::grammar::Expr::*; \n")
        if baseName != "Stmt":
            f.write("use crate::grammar::Stmt::*; \n")

        defineExpr(f, baseName, types.keys())
        for structName in types:
            defineStruct(structName, types, f)
            defineStructMethods(structName, types, f)


def main():
    outDir = sys.argv[1]
    defineAst(outDir, "Expr", {
      "Binary":["Expr:left","Token:operator","Expr:right"],
      "Grouping":["Expr:expression"],
      "Unary":["Token:operator","Expr:right"],
      "Variable" : ["Token:name"],
      "This" : ["Token:keyword"],
      "Assign":["Token:name","Expr:value"],
      "Get": ["Expr:object","Token:name"],
      "Set": ["Expr:object","Token:name","Expr:value"],
      "Super": ["Token:method","Token:keyword"],
      "Logical" : ["Expr:left","Token:operator","Expr:right"],
      "Call": ["Expr:callee"," Token:paren","Vec<Expr>:arguments"],
      "Lambda":["Token:paren","Vec<Token>:params","Vec<Stmt>:body"],
    });
    defineAst(outDir, "Stmt", {
      "Expression":["Expr:expr"],
      "Block":["Vec<Stmt>:statements"],
      "Class":["Token:name","Vec<Function>:methods","Option<Variable>:superclass"],
      "Function":["Token:name","Vec<Token>:params","Vec<Stmt>:body"],
      "Print":["Expr:expr"],
        "Var": ["Token:name","Option<Expr>:initializer"],
        "While": ["Expr:condition","Stmt:body"],
        "Break":[],
        "Continue": [],
        "If":["Expr:condition","Stmt:thenBranch","Option<Stmt>:elseBranch"],
        "Return": ["Token:keyword","Option<Expr>:value"],
    });

if __name__ == "__main__":
    main()