#[derive(PartialEq, Clone, Debug)]
pub struct Ident(pub String);

#[derive(PartialEq, Clone, Debug)]
pub enum Literal {
    String(String),
    Int(i32),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Expression {
    Ident(Ident),
    Literal(Literal),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Statement {
    LetExpression(Ident, Expression),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}
