#[derive(Debug, PartialEq, Clone)]

pub enum Statement {
  Expression(Expr),
  FunctionDef {
    name: String,
    params: Vec<String>,
    body: Vec<Statement>,
  },
  Return(Option<Box<Expr>>),
}
pub enum Expr {
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Literal(LiteralValue),

    // New: Variable reference like `x` or `count`
    Variable(String),
    // New: Assignment like `x = 5`
    Assign {
        name: String,
        value: Box<Expr>,
    },
    FunctionCall {
      name: String,
      args: Vec<Expr>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Number(i64),
}


