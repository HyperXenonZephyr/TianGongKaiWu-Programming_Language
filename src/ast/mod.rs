use crate::lexer::Span;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    VariableDecl(VariableDecl),
    Assignment(Assignment),
    Expression(Expression),
    IfStatement(IfStatement),
    LoopStatement(LoopStatement),
    WhileStatement(WhileStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    FunctionDecl(FunctionDecl),
    TryCatch(TryCatch),
    PrintStatement(PrintStatement),
    ImportStatement(ImportStatement),
    ExportStatement(ExportStatement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDecl {
    pub name: String,
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assignment {
    pub target: AssignmentTarget,
    pub value: Expression,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssignmentTarget {
    Identifier(String),
    MemberAccess(Box<MemberAccess>),
    IndexAccess(Box<IndexAccess>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Vec<Statement>,
    pub else_if_branches: Vec<ElseIfBranch>,
    pub else_branch: Option<Vec<Statement>>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElseIfBranch {
    pub condition: Expression,
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopStatement {
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakStatement {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Statement>,
    pub return_type: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    pub name: String,
    pub param_type: Option<String>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryCatch {
    pub try_block: Vec<Statement>,
    pub catch_blocks: Vec<CatchBlock>,
    pub finally_block: Option<Vec<Statement>>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatchBlock {
    pub exception_type: Option<String>,
    pub exception_name: String,
    pub statements: Vec<Statement>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintStatement {
    pub expressions: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatement {
    pub module: String,
    pub imports: Vec<ImportItem>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportItem {
    All,
    Specific(String),
    Alias(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportStatement {
    pub exports: Vec<ExportItem>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportItem {
    Function(String),
    Variable(String),
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(String, Span),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    MemberAccess(MemberAccess),
    IndexAccess(IndexAccess),
    Array(ArrayExpr),
    Dict(DictExpr),
    Parenthesized(Box<Expression>, Span),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    Number(String),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub left: Box<Expression>,
    pub op: BinaryOp,
    pub right: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallExpr {
    pub callee: Box<Expression>,
    pub args: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberAccess {
    pub object: Box<Expression>,
    pub member: String,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexAccess {
    pub object: Box<Expression>,
    pub index: Box<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayExpr {
    pub elements: Vec<Expression>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictExpr {
    pub entries: Vec<DictEntry>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictEntry {
    pub key: Expression,
    pub value: Expression,
    pub span: Span,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "加"),
            BinaryOp::Sub => write!(f, "減"),
            BinaryOp::Mul => write!(f, "乘"),
            BinaryOp::Div => write!(f, "除"),
            BinaryOp::Mod => write!(f, "餘"),
            BinaryOp::Pow => write!(f, "冪"),
            BinaryOp::Equal => write!(f, "等於"),
            BinaryOp::NotEqual => write!(f, "不等於"),
            BinaryOp::Greater => write!(f, "大於"),
            BinaryOp::Less => write!(f, "小於"),
            BinaryOp::GreaterEqual => write!(f, "大於等於"),
            BinaryOp::LessEqual => write!(f, "小於等於"),
            BinaryOp::And => write!(f, "且"),
            BinaryOp::Or => write!(f, "或"),
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Not => write!(f, "非"),
            UnaryOp::Neg => write!(f, "負"),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(n) => write!(f, "{}", n),
            Literal::String(s) => write!(f, "「{}」", s),
            Literal::Boolean(b) => write!(f, "{}", if *b { "真" } else { "假" }),
            Literal::Null => write!(f, "無"),
        }
    }
}

impl Expression {
    pub fn span(&self) -> Span {
        match self {
            Expression::Literal(_) => Span::new(0, 0, 0, 0),
            Expression::Identifier(_, span) => *span,
            Expression::Binary(expr) => expr.span,
            Expression::Unary(expr) => expr.span,
            Expression::Call(expr) => expr.span,
            Expression::MemberAccess(expr) => expr.span,
            Expression::IndexAccess(expr) => expr.span,
            Expression::Array(expr) => expr.span,
            Expression::Dict(expr) => expr.span,
            Expression::Parenthesized(_, span) => *span,
        }
    }
}

impl Statement {
    #[allow(dead_code)]
    pub fn span(&self) -> Span {
        match self {
            Statement::VariableDecl(decl) => decl.span,
            Statement::Assignment(assign) => assign.span,
            Statement::Expression(expr) => expr.span(),
            Statement::IfStatement(stmt) => stmt.span,
            Statement::LoopStatement(stmt) => stmt.span,
            Statement::WhileStatement(stmt) => stmt.span,
            Statement::BreakStatement(stmt) => stmt.span,
            Statement::ReturnStatement(stmt) => stmt.span,
            Statement::FunctionDecl(decl) => decl.span,
            Statement::TryCatch(stmt) => stmt.span,
            Statement::PrintStatement(stmt) => stmt.span,
            Statement::ImportStatement(stmt) => stmt.span,
            Statement::ExportStatement(stmt) => stmt.span,
        }
    }
}