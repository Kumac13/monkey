use std::process;
// expr = table_name ( "." method)*?
// table_name = [a-zA-Z_][a-zA-Z0-9_]*
// column_name = filed_name | tablename "." field_name
// field_name = [a-zA-Z0-9_]+
// method = select_method| where_method | limit_method | orderby_method | groupby_method | open_method
// select_method = ( "." "select(" column_name ( "," column_name )* ")" )*?
// where_method = ( "." "where("  ( where_args | where_arg ( "," where_arg )*) ")" )*?
// where_args = where_arg ( ( "and" | "or" ) where_arg )*?
// where_arg = column_name where_str_op where_term | column_name where_num_op num
// where_term = "'" ( [a-zA-Z0-9_%]* "%" [a-zA-Z0-9_%]* | [a-zA-Z0-9_%]* "_" [a-zA-Z0-9_%]* | [a-zA-Z0-9_%]+ ) "'" | num | "true" | "false" | "null"
// where_num_op = "=" | "!=" | "<" | "<=" | ">" | ">="
// where_str_op = "=" | "!=" | "LIKE" | "like"
// limit_method = ( "." "limit(" num ")" )?
// orderby_method = ( "." "orderby(" orderby_arg ( "," orderby_arg )* ")" )?
// orderby_arg = [a-zA-Z_][a-zA-Z0-9_]* ( " ASC" | " DESC" | "asc" | "desc" )?
// groupby_method = ( "." "groupby(" column_name ( "," column_name )* ")" )?
// open_method = ( "." "open()" )?
// num = [0-9]+

#[derive(Debug)]
pub enum AST {
    Table(String),
    Method(MethodType),
    Arg(String),
    Seq(Vec<AST>),
}

#[derive(Debug)]
pub enum MethodType {
    Select,
    Where,
    Limit,
    OrderBy,
    GroupBy,
    Open,
    Anything,
}

fn tokenize(expr: &str) -> AST {
    enum ParseStatus {
        Table,
        Method,
        MethodArg,
    }

    let mut p = expr.chars().peekable();

    let mut seq = Vec::new();
    let mut method_type = MethodType::Anything;

    let mut parse_state = ParseStatus::Table;

    let mut table_name = String::new();
    let mut method_name = String::new();
    let mut method_arg = String::new();
    let mut method_args = Vec::new();

    while let Some(c) = p.next() {
        match parse_state {
            ParseStatus::Table => {
                if c == '.' {
                    seq.push(AST::Table(table_name));
                    table_name = String::new();
                    parse_state = ParseStatus::Method;
                } else {
                    table_name.push(c);
                }
            }
            ParseStatus::Method => {
                if c == '(' {
                    match method_name.as_ref() {
                        "select" => method_type = MethodType::Select,
                        "where" => method_type = MethodType::Where,
                        "limit" => method_type = MethodType::Limit,
                        "orderby" => method_type = MethodType::OrderBy,
                        "groupby" => method_type = MethodType::GroupBy,
                        "open" => method_type = MethodType::Open,
                        _ => {
                            eprintln!("invalid method name {:?}", method_name);
                            process::exit(1);
                        }
                    }

                    method_name = String::new();
                    parse_state = ParseStatus::MethodArg;
                } else {
                    method_name.push(c);
                }
            }
            ParseStatus::MethodArg => {
                if c.is_whitespace() {
                    continue;
                }

                if c == ')' {
                    if let Some(c) = p.peek() {
                        if *c == '.' {
                            parse_state = ParseStatus::Method;
                            p.next();
                        } else {
                            continue;
                        }
                    }
                } else {
                    method_arg.push(c);

                    match method_type {
                        MethodType::Select | MethodType::GroupBy => {
                            if let Some(c) = p.peek() {
                                if *c == ',' {
                                    method_args.push(AST::Arg(method_arg));
                                    method_arg = String::new();
                                } else if *c == ')' {
                                    method_args.push(AST::Arg(method_arg));
                                    method_arg = String::new();
                                    seq.push(AST::Seq(vec![
                                        AST::Method(MethodType::Select),
                                        AST::Seq(method_args),
                                    ]));
                                    method_args = Vec::new();
                                }
                            }
                        }
                        MethodType::Where => {
                            if let Some(c) = p.peek() {
                                if *c == ')' {
                                    method_args.push(AST::Arg(method_arg));
                                    method_arg = String::new();
                                    seq.push(AST::Seq(vec![
                                        AST::Method(MethodType::Where),
                                        AST::Seq(method_args),
                                    ]));
                                    method_args = Vec::new();
                                }
                            }
                        }
                        MethodType::OrderBy => {
                            if let Some(c) = p.peek() {
                                if *c == ')' {
                                    method_args.push(AST::Arg(method_arg));
                                    method_arg = String::new();
                                    seq.push(AST::Seq(vec![
                                        AST::Method(MethodType::OrderBy),
                                        AST::Seq(method_args),
                                    ]));
                                    method_args = Vec::new();
                                }
                            }
                        }
                        MethodType::Limit => {
                            if let Some(c) = p.peek() {
                                if *c == ')' {
                                    if let Ok(i) = method_arg.parse::<i32>() {
                                        seq.push(AST::Seq(vec![
                                            AST::Method(MethodType::Limit),
                                            AST::Arg(i.to_string()),
                                        ]));
                                    } else {
                                        eprintln!("invalid limit value {}", method_arg);
                                        process::exit(1);
                                    }
                                } else {
                                    continue;
                                }
                            }
                        }
                        MethodType::Open => {
                            if !method_arg.is_empty() {
                                eprintln!("invalid open method");
                                process::exit(1);
                            } else {
                                seq.push(AST::Method(MethodType::Open));
                            }
                        }
                        MethodType::Anything => {
                            eprintln!("invalid method");
                            process::exit(1);
                        }
                    }
                }
            }
        }
    }
    AST::Seq(seq)
}

/*
fn get_soql(ast: &AST) -> Result<Method, Box<dyn std::error::Error>> {
    match ast {
        AST::Table(table_name) => Ok(Method::Select(vec![table_name.to_string()])),
        AST::Method(method_name, args) => match method_name.as_ref() {
            "select" => Ok(Method::Select(args.iter().map(|x| x.to_string()).collect())),
            "where" => Ok(Method::Where(args.iter().map(|x| x.to_string()).collect())),
            "limit" => Ok(Method::Limit(args.iter().map(|x| x.to_string()).collect())),
            "orderby" => Ok(Method::OrderBy(
                args.iter().map(|x| x.to_string()).collect(),
            )),
            "groupby" => Ok(Method::GroupBy(
                args.iter().map(|x| x.to_string()).collect(),
            )),
            "open" => Ok(Method::Open),
            _ => Err("method not found".into()),
        },
    }
}
*/

fn main() {
    let query =
        "Opportunity.select(Id, Name, Account.Name).where(StageName = 'Closed Won').limit(10)";

    // parse
    let ast = tokenize(query);

    println!("{:?}", ast);

    /*
    // generate
    let soql = get_soql(&ast);

    println!("{:?}", soql);
    */
}
