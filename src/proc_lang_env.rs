use std::rc::Rc;
use std::fmt;
use int_bool_proc::*;
use proc_lang_exp::ProcLangExp;

#[derive(Debug,Clone)]
pub enum ProcEnvExp {
    EmptyEnv,
    ExtendEnv(String, IntBoolProc, Rc<ProcEnvExp>),
    ExtendEnvRec(String, String, Rc<ProcLangExp>, Rc<ProcEnvExp>),
}

impl ProcEnvExp {
    pub fn new_env() -> Self {
        ProcEnvExp::EmptyEnv
    }
    pub fn extend_env(&self, s:&String, val: IntBoolProc) -> Self {
        ProcEnvExp::ExtendEnv(s.clone(), val, Rc::new(self.clone()))
    }
    pub fn extend_env_rec(&self, pname: &String, bvar: &String, pbody:ProcLangExp) -> Self{
        ProcEnvExp::ExtendEnvRec(pname.clone(), bvar.clone(), Rc::new(pbody.clone()) ,Rc::new(self.clone()))

    }
    pub fn apply_env(&self, s:&String) -> Option<IntBoolProc> {
        match self.clone() {
            ProcEnvExp::ExtendEnv(var, val, env) =>
                                       if s[..] == var[..] {
                                        Some(val)
                                       } else {
                                        env.apply_env(s)
                                        },
            ProcEnvExp::ExtendEnvRec(pname, bvar, pbody, env) =>
                                        if s[..] == pname[..] {
                                            Some(IntBoolProc::Procedure(
                                                bvar,
                                                Rc::new((*pbody).clone()),
                                                Rc::new((*self).clone())
                                            ))}
                                        else { self.apply_env(s)}

            ProcEnvExp::EmptyEnv => None,
        }}
    pub fn is_null_env(&self) -> bool {
        match self.clone() {
            ProcEnvExp::EmptyEnv  => true,
            _                     => false,
        }}
    pub fn to_string(&self) -> String {
        match self.clone() {
            ProcEnvExp::EmptyEnv => "[]".to_string(),
            ProcEnvExp::ExtendEnv(var, val, env) => {let mut temp = "[".to_string();
                                                temp.push_str(&(var.to_string()));
                                                temp.push_str(&(", ".to_string()));
                                                temp.push_str(&(val.to_string()));
                                                temp.push_str(&(" ".to_string()));
                                                temp.push_str(&(env.to_string()));
                                                temp.push_str(&("]".to_string()));
                                                temp},
            ProcEnvExp::ExtendEnvRec(pname, bvar, pbody, env) => {let mut temp = "proc ".to_string();
                temp.push_str(&(bvar.to_string()));
                temp.push_str(&(" <<".to_string()));
                temp.push_str(&(pbody.to_string()));
                temp.push_str(&(">> ".to_string()));
                temp.push_str(&(env.to_string()));
                temp},

        }}
}

impl fmt::Display for ProcEnvExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("");
        let s1 = self.to_string();
        s.push_str(&s1);
        write!(f, "{}", s)
    }}

#[cfg(test)]
mod test {
    use super::ProcEnvExp;
    use let_lang_exp::*;

    #[test]
    fn basic_tests() {
        let null_env = ProcEnvExp::new_env();
        assert!(null_env.is_null_env());

        let env2 = null_env.extend_env(&("var1".to_string()), 25);
        assert!(!(env2.is_null_env()));
    }
}
