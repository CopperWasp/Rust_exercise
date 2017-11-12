// Let-language Expressions

use std::rc::Rc; // Rc<T> reference counted pointer type over immutable value
use std::fmt;
use proc_lang_env;

// data type for abstract-syntax tree
#[derive(Debug,Clone)]
pub enum ProcLangExp {  // set of possible LetLangExp's
    ConstExp(i32),
    Boolean(bool),
    DiffExp(Rc<ProcLangExp>, Rc<ProcLangExp>),
    PlusExp(Rc<ProcLangExp>, Rc<ProcLangExp>),
    IsZeroExp(Rc<ProcLangExp>),
    IfExp(Rc<ProcLangExp>, Rc<ProcLangExp>, Rc<ProcLangExp>),
    VarExp(String),
    LetExp(String, Rc<ProcLangExp>, Rc<ProcLangExp>),
    ProcExp(String, Rc<ProcLangExp>),
    CallExp(Rc<ProcLangExp>, Rc<ProcLangExp>),
    //LetRecExp(String, String, Rc<ProcLangExp>, Rc<ProcLangExp>), //check
}

// create a constructor and to_string() method for each type of LetLangExp
impl ProcLangExp {
    pub fn new_const_exp(num: i32) -> Self {
        ProcLangExp::ConstExp(num)
    }
    pub fn new_boolean(tv: bool) -> Self {
        ProcLangExp::Boolean(tv)
    }
    pub fn new_diff_exp(arg1: &ProcLangExp, arg2: &ProcLangExp) -> Self {
        ProcLangExp::DiffExp(Rc::new(arg1.clone()), Rc::new(arg2.clone()))
    }
    pub fn new_iszero(arg: &ProcLangExp) -> Self {
        ProcLangExp::IsZeroExp(Rc::new(arg.clone()))
    }
    pub fn new_if_exp(arg1: &ProcLangExp, arg2: &ProcLangExp, arg3: &ProcLangExp) -> Self {
        ProcLangExp::IfExp(Rc::new(arg1.clone()), Rc::new(arg2.clone()), Rc::new(arg3.clone()))
    }
    pub fn new_var_exp(s: &String) -> Self {
        ProcLangExp::VarExp(s.clone())
    }
    pub fn new_let_exp(s: &String, arg1: &ProcLangExp, arg2: &ProcLangExp) -> Self {
        ProcLangExp::LetExp(s.clone(), Rc::new(arg1.clone()), Rc::new(arg2.clone()))
    }

    pub fn new_plus_exp(arg1: &ProcLangExp, arg2: &ProcLangExp) -> Self {
        ProcLangExp::PlusExp(Rc::new(arg1.clone()), Rc::new(arg2.clone()))
    }
    pub fn new_proc_exp(s: &String, arg: &ProcLangExp) -> Self {
        ProcLangExp::ProcExp(s.clone(), Rc::new(arg.clone()))
    }
    pub fn new_call_exp(arg1: &ProcLangExp, arg2: &ProcLangExp) -> Self{
        ProcLangExp::CallExp(Rc::new(arg1.clone()), Rc::new(arg2.clone()))
    }
    /*pub fn new_letrec_exp(s1: &String, s2:String, arg1: &ProcLangExp, arg2: &ProcLangExp) -> Self{
        ProcLangExp::LetRecExp(s1.clone(), s2.clone(), Rc::new(arg1.clone()), Rc::new(arg2.clone()))
    }*/

    // a string representation, to be used by the formatter, for each type of LetLangExp
    pub fn to_string(&self) -> String {
        match self.clone() {
            ProcLangExp::ConstExp(int)       => int.to_string(),
            ProcLangExp::Boolean(bool)       => bool.to_string(),
            ProcLangExp::DiffExp(e1, e2)     => {let mut temp: String = "-(".to_string();
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(", ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp.push_str(&(")".to_string()));
                                                temp}
            ProcLangExp::PlusExp(e1, e2)     => {let mut temp: String = "-(".to_string();
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(", ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp.push_str(&(")".to_string()));
                                                temp}
            ProcLangExp::IsZeroExp(e)        => {let mut temp = "iszero(".to_string();
                                                temp.push_str(&(e.to_string()));
                                                temp.push_str(&(")".to_string()));
                                                temp}
            ProcLangExp::IfExp(e1, e2, e3)   => {let mut temp = "if ".to_string();
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(" then ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp.push_str(&(" else ".to_string()));
                                                temp.push_str(&(e3.to_string()));
                                                temp}
            ProcLangExp::VarExp(var)         => var,
            ProcLangExp::LetExp(v, e1, e2)   => {let mut temp = "let ".to_string();
                                                temp.push_str(&(v.to_string()));
                                                temp.push_str(&(" = ".to_string()));
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(" in ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp}
            ProcLangExp::CallExp(e1, e2)    => {let mut temp= "(".to_string();
                                                temp.push_str(&(e1.to_string()));
                                                temp.push_str(&(" ".to_string()));
                                                temp.push_str(&(e2.to_string()));
                                                temp.push_str(&(")".to_string()));
                                                temp}
            ProcLangExp::ProcExp(v, e)      => {let mut temp= "proc(".to_string();
                                                temp.push_str(&(v.to_string()));
                                                temp.push_str(&(") ".to_string()));
                                                temp.push_str(&(e.to_string()));
                                                temp}
        }
    }
}

impl fmt::Display for ProcLangExp { // do not change this code
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("");
        let s1 = self.to_string();
        s.push_str(&s1);
        write!(f, "{}", s)
    }}


