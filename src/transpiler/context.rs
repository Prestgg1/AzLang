use std::collections::HashSet;

pub struct Context {
    pub mutable_vars: HashSet<String>,
    pub used_as_assignment: HashSet<String>,
    pub functions: Vec<String>,
    pub main_body: Vec<String>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            mutable_vars: HashSet::new(),
            used_as_assignment: HashSet::new(),
            functions: Vec::new(),
            main_body: Vec::new(),
        }
    }
    pub fn nested(&self) -> Context {
        Context {
            mutable_vars: self.mutable_vars.clone(),
            used_as_assignment: self.used_as_assignment.clone(),
            functions: self.functions.clone(),
            main_body: Vec::new(),
        }
    }
}
