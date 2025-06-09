#[derive(Debug, Clone)]
pub enum BuiltInFunction {
    Print,
    Input,
    Len,
    Number,
}

#[derive(Debug, Clone)]
pub enum Expr {
    String(String),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    Return(Box<Expr>),
    BuiltInCall {
        func: BuiltInFunction,
        args: Vec<Expr>,
    },
    MutableDecl {
        name: String,
        typ: String,
        value: Box<Expr>,
    },
    ConstantDecl {
        name: String,
        typ: String,
        value: Box<Expr>,
    },
    VariableRef(String),
    List(Vec<Expr>),
    FunctionDef {
        name: String,
        params: Vec<(String, String)>,
        body: Vec<Expr>,
    },
}

#[derive(Debug)]
pub struct Program {
    pub expressions: Vec<Expr>,
}
