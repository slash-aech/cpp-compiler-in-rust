// src/ast.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Expr {
    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),

    // Variable reference
    Variable {
        name: String,
        line: usize,
    },

    // Binary operations
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
        line: usize,
    },

    // Unary operations
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
        line: usize,
    },

    // Function call
    Call {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
        line: usize,
    },

    // Grouping (parentheses)
    Grouping(Box<Expr>),

    // Conditional expression
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
        line: usize,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    // Expression statement
    Expression(Expr),

    // Variable declaration
    Let {
        name: String,
        type_annotation: Option<Type>,
        initializer: Expr,
        line: usize,
    },

    // Return statement
    Return {
        value: Option<Expr>,
        line: usize,
    },

    // Block of statements
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Vec<Stmt>,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    pub statements: Vec<Stmt>,
}
