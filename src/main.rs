extern crate let_lang_proj;

use let_lang_proj::proc_lang_scanner::*;
use let_lang_proj::let_lang_parser::*;
use let_lang_proj::proc_lang_exp::*;
use let_lang_proj::proc_lang_env::*;
use let_lang_proj::int_bool_proc::*;
use std::rc::Rc;

fn value_of(ast: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc> { // defined in int_bool_proc.rs
    match ast.clone() {
        ProcLangExp::ConstExp(int)    => Some(IntBoolProc::Integer(int)),
        ProcLangExp::Boolean(b)       => Some(IntBoolProc::Boolean(b)),
        ProcLangExp::DiffExp(e1, e2)  => value_of_diff_exp(&(*e1), &(*e2), env),
        ProcLangExp::IsZeroExp(e)     => value_of_iszero(&(*e), env),
        ProcLangExp::IfExp(e1, e2, e3)  => value_of_if(&(*e1), &(*e2), &(*e3), env),
        ProcLangExp::VarExp(s)        => env.apply_env(&s),
        ProcLangExp::LetExp(s, e1, e2)  => value_of_let(&s, &(*e1), &(*e2), env),

        ProcLangExp::PlusExp(e1, e2) => value_of_plus_exp(&(*e1), &(*e2), env),
        ProcLangExp::ProcExp(s, body) => value_of_proc_exp(&s, &(*body), env),
        ProcLangExp::CallExp(e1, e2) => value_of_call_exp(&(*e1), &(*e2), env),

    }}
fn value_of_let(s: &String, e1: &ProcLangExp, e2: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc>{
    let new_val = value_of(e1, env);
    if new_val.is_none() { return None};
    let new_env = env.extend_env(s, new_val.unwrap());
    value_of(e2, &new_env)
}
fn value_of_if(e1: &ProcLangExp, e2: &ProcLangExp, e3: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc> {
    if match value_of(e1, env) { // compute value of test and treat as true only if boolean true
        Some(x) => match x {
            IntBoolProc::Integer(_i) => false,
            IntBoolProc::Procedure(_s, _e1, _e2) => false,
            IntBoolProc::Boolean(b) => b,
            },
        None => false,
        }
        {
            value_of(e2, env)
        } else {
            value_of(e3, env)
        }}
fn value_of_iszero(e: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc> {
    let opt_val = value_of(e, env);
    match opt_val {
        Some(x) => match x {
            IntBoolProc::Integer(i)  => Some(IntBoolProc::Boolean(i == 0)),
            IntBoolProc::Boolean(_b) => None,
            IntBoolProc::Procedure(_s, _e1, _e2) => None,
        },
        None    => None,
    }}
// checked difference
fn value_of_diff_exp(arg1: &ProcLangExp, arg2: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc> {
    let val1 = value_of(arg1, env);
    let val2 = value_of(arg2, env);
    if val1.is_none() || val2.is_none() {
        None
    } else {
        let v1 = val1.unwrap();
        let v2 = val2.unwrap();
        Some(IntBoolProc::Integer(value_of_diff_exp_work(&v1, &v2)))
    }}
fn value_of_diff_exp_work(a1: &IntBoolProc, a2: &IntBoolProc) -> i32 {
    let a1_int_val: i32 = match *a1 {
                            IntBoolProc::Integer(i) => i,
                            _                   => 0,
                            };
    let a2_int_val: i32 = match *a2 {
                            IntBoolProc::Integer(i) => i,
                            _                   => 0,
                            };
    a1_int_val - a2_int_val
}

// added for proc and letrec
fn show_parse(s: &String) -> Result <ProcLangExp, ParseErr> {
    println!("Parsing: {}", s);
    let tok_result= tokenize(s);
    match tok_result{
        Ok(v) => {
            let ast = try! (parse(&v));
            println!("AST: \n{:#?}\n", ast);
            println!("Stringified AST: {}\n", ast);
            Ok(ast)},
        Err(_e) => Err(ParseErr { message: "Token parse error.".to_string()})
    }

}
fn value_of_plus_exp(arg1: &ProcLangExp, arg2: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc> {
    let val1 = value_of(arg1, env);
    let val2 = value_of(arg2, env);
    if val1.is_none() || val2.is_none() {
        None
    } else {
        let v1 = val1.unwrap();
        let v2 = val2.unwrap();
        Some(IntBoolProc::Integer(value_of_plus_exp_work(&v1, &v2)))
    }}
fn value_of_plus_exp_work(a1: &IntBoolProc, a2: &IntBoolProc) -> i32 {
    let a1_int_val: i32 = match *a1 {
        IntBoolProc::Integer(i) => i,
        _                   => 0,
    };
    let a2_int_val: i32 = match *a2 {
        IntBoolProc::Integer(i) => i,
        _                   => 0,
    };
    a1_int_val + a2_int_val
}
fn value_of_proc_exp(v: &String, body: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc> {
    Some(IntBoolProc::Procedure(v.clone(), Rc::new(body.clone()), Rc::new(env.clone())))
}
fn value_of_call_exp(rator: &ProcLangExp, rand: &ProcLangExp, env: &ProcEnvExp) -> Option<IntBoolProc> {
    let opt_closure=
        match rator.clone() {
            ProcLangExp::VarExp(s) => env.apply_env(&s),
            ProcLangExp::ProcExp(v, e) => Some(IntBoolProc::Procedure(v.clone(), Rc::new(((*e). clone())), Rc::new(env.clone()))),
            _ => None,
        };
    let opt_evald_rand = value_of(rand ,env);
    if opt_closure.is_none() || opt_evald_rand.is_none() {
        None
    } else {
        apply_procedure(&(opt_closure.unwrap()), &(opt_evald_rand.unwrap()))
    }
}
fn apply_procedure(closure: &IntBoolProc, rand: &IntBoolProc) -> Option<IntBoolProc>{
    match closure.clone() {
        IntBoolProc::Procedure(v, b, env) =>
            value_of(&(*b).clone(), &(env.clone()).extend_env(&(v.to_string()), rand.clone())),
                                        _ => None,
    }
}

#[allow(dead_code)]
fn main() {
    println!("\nStarting to parse: milestone let");
    let mile_str = "let x = 200 in let f = proc (z) -(z, x) in let x = 100 in let g = proc (z) -(z, x) in -((f 1), (g 1))";
    // compute tokens
    let tok_result = tokenize(mile_str);        // returns Result<Vec<Token>, LexErr>
    let mile_tokens = tok_result.unwrap();

    // compute AST and evaluate
    let env = ProcEnvExp::EmptyEnv;
    let mile_ast_result = parse(&mile_tokens);  // returns Result<LetLangExp, ParseErr>
    match mile_ast_result {
        Ok(v)  => {println!("{}", v);      // regular print
            println!("{:#?}", v);   // pretty-print in debug format
            println!("\nmilestone = {:?}", value_of(&v, &env));},
        Err(e) => println!("Syntax error: {:#?}", e),
    }
}
