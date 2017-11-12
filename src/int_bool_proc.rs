use std::rc::Rc;
use proc_lang_exp::ProcLangExp;
use proc_lang_env::ProcEnvExp;

#[derive(Debug,Clone)]
pub enum IntBoolProc {
    Integer(i32),
    Boolean(bool),
    Procedure(String, Rc<ProcLangExp>, Rc<ProcEnvExp>),
}

impl IntBoolProc {
    pub fn to_string(&self) -> String {
        match (*self).clone() {
            IntBoolProc::Integer(i) => i.to_string(),
            IntBoolProc::Boolean(b) => b.to_string(),
            IntBoolProc::Procedure(s, exp, env) => s.to_string(), //check
        }
    }
}