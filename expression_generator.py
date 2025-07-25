import sys
import os

def main():
    if len(sys.argv) != 2:
        print("Usage: generate_ast <output directory>", file=sys.stderr)
        sys.exit(64)

    output_dir = sys.argv[1]
    define_ast(output_dir, "Expr", [
        "Binary   -> pub left: Box<Expr>, pub operator: Token, pub right: Box<Expr>,",
        "Grouping -> pub expression: Box<Expr>,",
        "Literal  -> pub value: LiteralValue,",
        "Unary    -> pub operator: Token, pub right: Box<Expr>,",
        "Variable -> pub name: Token,",
    ])

    define_ast(output_dir, "Stmt", [
        "Expression -> pub expression: Box<Expr>",
        "Print      -> pub expression: Box<Expr>",
        "Var        -> pub name: Token, pub initializer: Box<Expr>"
    ])

def define_ast(output_dir, base_name, types):
    path = os.path.join(output_dir, f"{base_name}.rs")
    with open(path, "w") as f:
        f.write("use crate::expr::Expr;\n")
        f.write("use crate::token::Token;\n")
        f.write("use crate::token_type::LiteralValue;\n\n")
        f.write("#[derive(Clone)]\n")
        f.write(f"pub enum {base_name} {{ \n")
        for type_def in types:
            expr = type_def.split("->")[0].strip()
            f.write(f"    {expr}({expr}),\n")
        f.write("}\n\n")

        for type_def in types:
            expr = type_def.split("->")[0].strip()
            fields = type_def.split("->")[1].strip() if "->" in type_def else ""
            define_type(f, base_name, expr, fields)

        define_visitor(f, base_name, [type_def.split("->")[0].strip() for type_def in types])

        define_implementation(f, base_name, [type_def.split("->")[0].strip() for type_def in types])


def define_type(f, base_name, class_name, field_list):
    f.write("#[derive(Clone)]\n")
    f.write(f"pub struct {class_name} {{\n")
    f.write(f"  {field_list}\n")
    f.write("}\n\n")


def define_visitor(f, base_name, types):
    f.write(f"pub trait Visitor<R> {{\n")
    for type_name in types:
        f.write(f"    fn visit_{type_name.lower()}_{base_name.lower()}(&self, expr: &{type_name}) -> R;\n")
    f.write("}\n\n")

def define_implementation(f, base_name, types):
    f.write(f"impl {base_name} {{\n")
    f.write(f"    pub fn accept<V: Visitor<R>, R>(&self, visitor: &V) -> R {{\n")
    f.write(f"        match self {{\n")
    for type_name in types:
        f.write(f"            {base_name}::{type_name}(expr) => visitor.visit_{type_name.lower()}_expr(expr),\n")
    f.write("        }\n")
    f.write("    }\n")
    f.write("}\n\n")


if __name__ == "__main__":
    main()
