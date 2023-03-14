#[derive(Debug, Clone)]
pub enum NodeType {
    // STATEMENTS
    Program,
    VarDeclaration,
    FunctionDeclaration,
    BlockStatement,
    // EXPRESSIONS
    AssignmentExpression,
    BinaryExpression,
    MemberExpression,
    CallExpression,
    Identifier,
    Property,
    ObjectLiteral,
    NumbericLiteral,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Program(Program),
    BlockStatement(BlockStatement),
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub kind: NodeType,
    pub start: usize,
    pub end: usize,
    pub body: Vec<Statement>,
}

impl Program {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::Program,
            start,
            end,
            body: Vec::new(),
        }
    }
    pub fn append(&mut self, stmt: Statement) {
        self.body.push(stmt);
    }
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    kind: NodeType,
    start: usize,
    end: usize,
    body: Vec<Statement>,
}

impl BlockStatement {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::BlockStatement,
            start,
            end,
            body: Vec::new(),
        }
    }
    pub fn append(&mut self, stmt: Statement) {
        self.body.push(stmt);
    }
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Var(VarDeclaration),
    Function(FunctionDeclaration),
}
#[derive(Debug, Clone)]
pub struct VarDeclaration {
    kind: NodeType,
    constant: bool,
    identifier: Identifier,
    value: Option<Expression>,
    start: usize,
    end: usize,
}

impl VarDeclaration {
    pub fn new(
        ident: Identifier,
        value: Option<Expression>,
        constant: bool,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind: NodeType::VarDeclaration,
            identifier: ident,
            constant,
            value,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    kind: NodeType,
    parameters: Vec<Identifier>,
    name: Identifier,
    body: BlockStatement,
    start: usize,
    end: usize,
}

impl FunctionDeclaration {
    pub fn new(
        name: Identifier,
        params: Vec<Identifier>,
        body: BlockStatement,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind: NodeType::FunctionDeclaration,
            parameters: params,
            name,
            body,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Assignment(Box<AssignmentExpression>),
    Binary(Box<BinaryExpression>),
    Member(Box<MemberExpression>),
    Call(Box<CallExpression>),
    Identifier(Box<Identifier>),
    Property(Box<Property>),
    ObjectLiteral(Box<ObjectLiteral>),
    NumbericLiteral(Box<NumbericLiteral>),
}

#[derive(Debug, Clone)]
pub struct AssignmentExpression {
    kind: NodeType,
    assinge: Expression,
    value: Expression,
    start: usize,
    end: usize,
}

impl AssignmentExpression {
    pub fn new(assinge: Expression, value: Expression, start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::AssignmentExpression,
            assinge,
            value,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    kind: NodeType,
    left: Expression,
    right: Expression,
    operator: String,
    start: usize,
    end: usize,
}

impl BinaryExpression {
    pub fn new(
        left: Expression,
        right: Expression,
        operator: String,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind: NodeType::BinaryExpression,
            left,
            right,
            operator,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemberExpression {
    kind: NodeType,
    object: Expression,
    property: Expression,
    computed: bool,
    start: usize,
    end: usize,
}
impl MemberExpression {
    pub fn new(
        object: Expression,
        property: Expression,
        computed: bool,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind: NodeType::MemberExpression,
            object,
            property,
            computed,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    kind: NodeType,
    caller: Expression,
    arguments: Vec<Expression>,
    start: usize,
    end: usize,
}

impl CallExpression {
    pub fn new(caller: Expression, args: Vec<Expression>, start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::CallExpression,
            arguments: args,
            caller,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    kind: NodeType,
    symbol: String,
    start: usize,
    end: usize,
}

impl Identifier {
    pub fn new(symbol: String, start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::Identifier,
            symbol,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Property {
    kind: NodeType,
    key: Identifier,
    value: Option<Expression>,
    start: usize,
    end: usize,
}

impl Property {
    pub fn new(key: Identifier, value: Option<Expression>, start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::Identifier,
            key,
            value,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ObjectLiteral {
    kind: NodeType,
    properties: Vec<Property>,
    start: usize,
    end: usize,
}

impl ObjectLiteral {
    pub fn new(properties: Vec<Property>, start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::ObjectLiteral,
            properties,
            start,
            end,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NumbericLiteral {
    kind: NodeType,
    value: i64,
    start: usize,
    end: usize,
}

impl NumbericLiteral {
    pub fn new(value: i64, start: usize, end: usize) -> Self {
        Self {
            kind: NodeType::NumbericLiteral,
            value,
            start,
            end,
        }
    }
}
