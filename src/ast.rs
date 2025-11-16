#[derive(Debug, PartialEq, Clone)]
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


