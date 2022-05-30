use colored::Colorize;
use super::parser::Type;


const SPACE: usize = 4;
fn sep(depth: usize) -> String {
    " ".repeat(SPACE * depth)
}

fn listArgs(args: &Vec<Type>, sep: &str, br_depth: usize) -> String {
    args.iter().map(|a| a.to_string(0, br_depth)).collect::<Vec<String>>().join(sep)
}

fn listBlock(args: &Vec<Type>, depth: usize) -> String {
    args.iter().map(|a| format!("{}{}", sep(depth), a.to_string(depth, 0))).collect::<Vec<String>>().join("\n")
}


fn listBoxedBlock(args: &Vec<Box<Type>>, depth: usize) -> String {
    args.iter().map(|a| format!("{}{}", sep(depth), a.to_string(depth, 0))).collect::<Vec<String>>().join("\n")
}


fn bracket(br: &str, depth: usize) -> String {
    if (depth + 1) % 2 == 0 {
        br.magenta()
    } else if (depth + 1) % 3 == 0{
        br.cyan()
    } else {
        br.yellow()
    }.to_string()
}


impl Type {
    pub fn fn_symbol(&self, depth: usize, br_depth: usize) -> String {
        match &self {
            Type::Symbol(n) => n.to_string(),
            _ => self.to_string(depth, br_depth)
        }
    }

    pub fn to_string(&self, depth: usize, br_depth: usize) -> String {
        match &self {
            Type::Number(n) => n.to_string().yellow().to_string(),
            Type::Bool(n) => n.to_string().yellow().to_string(),
            Type::Symbol(n) => {
                if n == "true" || n == "false" {
                    n.yellow()
                } else {
                    n.red()
                }.to_string()
            },
            Type::String(n) => format!("\"{}\"", n).green().to_string(),
            Type::Call { function, arguments } => {
                let func_name = function.fn_symbol(depth, br_depth);

                let op: Option<&str> = match func_name.as_str() {
                    "__add__" => Some(" + "),
                    "__sub__" => Some(" - "),
                    "__mul__" => Some(" * "),
                    "__div__" => Some(" / "),
                    "__mod__" => Some(" % "),
                    "__pow__" => Some(" ** "),
                    "__lt__" => Some(" < "),
                    "__gt__" => Some(" > "),
                    "__le__" => Some(" <= "),
                    "__ge__" => Some(" >= "),
                    "__eq__" => Some(" == "),
                    "__ne__" => Some(" != "),
                    "__and__" => Some(" && "),
                    "__or__" => Some(" || "),
                    "__not__" => Some("!"),
                    _ => None
                };

                if let Some(op) = op {
                    if op == "!" {
                        format!("!{}{}{}", bracket("(", br_depth), listArgs(arguments, op, br_depth+1), bracket(")", br_depth))
                    } else {
                        format!("{}{}{}", bracket("(", br_depth), listArgs(arguments, op, br_depth+1), bracket(")", br_depth))
                    }
                } else {
                    format!("{}{}{}{}", func_name.blue(), bracket("(", br_depth), listArgs(arguments, ", ", br_depth+1), bracket(")", br_depth))
                }

            },
            Type::VariableDeclaration { variable, value } => format!("{} {} = {}", "let".purple(), variable.to_string(depth, br_depth), value.to_string(depth, br_depth)),
            Type::Assignment { variable, value } => format!("{} = {}", variable.to_string(depth, br_depth), value.to_string(depth, br_depth)),
            Type::CreateFunction { name, code, parameters } => format!("{} {}{}{}{} {}\n{}\n{}{}", "fn".purple(), name.to_string(depth, br_depth), bracket("(", br_depth), parameters.iter().map(|p| p.red().to_string()).collect::<Vec<String>>().join(", "),bracket(")", br_depth), bracket("{", depth), listBoxedBlock(code, depth + 1), sep(depth), bracket("}", depth)),
            Type::Conditional { condition, then, otherwise } => {
                let first = format!("{} {} {}\n{}\n{}{}", "if".purple(),  condition.to_string(depth, br_depth), bracket("{", depth), listBlock(then, depth + 1), sep(depth), bracket("}", depth));
                if let Some(other) = otherwise {
                    return format!("{} {} {}\n{}\n{}{}", first, "else".purple(), bracket("{", depth), listBlock(other, depth+1), sep(depth), bracket("}", depth))
                } 
                return first;
            },
            Type::While { condition, code } => format!("{} {} {}\n{}\n{}{}", "while".purple(), condition.to_string(depth, br_depth), bracket("{", depth), listBlock(code, depth + 1), sep(depth), bracket("}", depth)),
            Type::Comment(comment) => format!("//{}", comment).bright_black().to_string(),
            // Type::Function(_) => todo!(),
            // Type::Invocation { code } => todo!(),
            _ => format!("{:?}", self)
        }
    }
}
