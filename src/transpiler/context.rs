use std::collections::{HashMap, HashSet};

use crate::parser::ast::Type;

pub struct TranspileContext {
    pub imports: HashSet<String>,
    pub symbol_types: HashMap<String, Type>,
    pub needs_allocator: bool,
    pub uses_stdout: bool,
    pub mutable_symbols: HashSet<String>,
    pub used_input_fn: bool,
    pub cleanup_statements: Vec<String>,
    pub used_sum_fn: bool,
    pub used_split_n_fn: bool,
    pub used_split_auto_fn: bool,
}

impl TranspileContext {
    pub fn new() -> Self {
        Self {
            imports: HashSet::new(),
            symbol_types: HashMap::new(),
            needs_allocator: false,
            uses_stdout: false,
            mutable_symbols: HashSet::new(),
            used_input_fn: false,
            cleanup_statements: Vec::new(),
            used_sum_fn: false,
            used_split_n_fn: false,
            used_split_auto_fn: false,
        }
    }

    pub fn add_import(&mut self, import: &str) -> Option<String> {
        if self.imports.contains(import) {
            None
        } else {
            self.imports.insert(import.to_string());
            Some(import.to_string())
        }
    }
}
