use crate::frontend::lexer::{Token, TokenType};

mod ast;

use self::ast::{
    AssignmentExpression, BinaryExpression, Expression, Identifier, NumbericLiteral, Program,
    Statement,
};

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }
    fn not_eof(&self) -> bool {
        if let Some(token) = self.tokens.first() {
            return token.kind != TokenType::EOF;
        }
        false
    }
    // 获取第一个token
    fn at(&self) -> &Token {
        self.tokens.first().unwrap()
    }
    // 移出第一个token
    fn eat(&mut self) -> Token {
        self.tokens.remove(0)
    }
    fn expect(&self, kind: TokenType, err: &str) {
        let cur_token = self.at();
        if cur_token.kind != kind {
            panic!("{}", err);
        }
    }
    pub fn parse(&mut self) -> Statement {
        let mut program = Program::new(0, 0);
        while self.not_eof() {
            program.append(self.parse_statement());
        }

        Statement::Program(program)
    }
    fn parse_statement(&mut self) -> Statement {
        let cur_token = self.at();
        match cur_token.kind {
            TokenType::Let => {
                todo!()
            }
            TokenType::Const => {
                todo!()
            }
            TokenType::Fn => {
                todo!()
            }
            _ => {
                let expression = self.parse_expression();
                Statement::Expression(expression)
            }
        }
    }
    // 表达式优先级 从低到高
    // 赋值表达式
    // 对象表达式
    // 加法表达式   ✅
    // 乘法表达式   ✅
    // 成员访问
    // 方法调用
    // 数值、及括号等  ✅
    fn parse_expression(&mut self) -> Expression {
        self.parse_assignment_expression()
    }
    // 赋值表达式
    fn parse_assignment_expression(&mut self) -> Expression {
        println!("before: {:?}", self.at());
        let left = self.parse_additive_expression();

        println!("after: {:?}", self.at());
        if self.at().kind == TokenType::Equals {
            self.eat();

            // 连续赋值情况
            let value = self.parse_assignment_expression();

            let assignment_expression = AssignmentExpression::new(left, value, 0, 0);

            return Expression::Assignment(Box::new(assignment_expression));
        }

        left
    }
    // 加减等表达式
    fn parse_additive_expression(&mut self) -> Expression {
        let mut left = self.parse_multiplicitave_expression();

        while self.at().value == "+" || self.at().value == "-" {
            let operator = self.eat().value;
            let right = self.parse_multiplicitave_expression();
            let binary_expression = BinaryExpression::new(left, right, operator, 0, 0);
            left = Expression::Binary(Box::new(binary_expression));
        }
        left
    }
    // 乘除等表达式
    fn parse_multiplicitave_expression(&mut self) -> Expression {
        let mut left = self.parse_primary_expression();
        while self.at().value == "*" || self.at().value == "/" || self.at().value == "%" {
            let operator = self.eat().value;
            let right = self.parse_primary_expression();
            let binary_expression = BinaryExpression::new(left, right, operator, 0, 0);
            left = Expression::Binary(Box::new(binary_expression));
        }
        left
    }
    // 数值、符号解析
    fn parse_primary_expression(&mut self) -> Expression {
        let token_type = self.at().kind;
        match token_type {
            TokenType::Identifier => {
                let token = self.eat();
                let ident = Identifier::new(token.value, token.start, token.end);
                Expression::Identifier(Box::new(ident))
            }
            TokenType::Number => {
                let token = self.eat();
                let literal = NumbericLiteral::new(
                    token.value.parse::<i64>().unwrap(),
                    token.start,
                    token.end,
                );
                Expression::NumbericLiteral(Box::new(literal))
            }
            TokenType::OpenParen => {
                self.eat();
                let expression = self.parse_expression();
                self.expect(TokenType::CloseParen, "Unexpected token found inside parenthesised expression. Expected closing parenthesis.");
                self.eat();
                expression
            }
            _ => {
                panic!("Unexpected token found during parsing!, {:?}", self.at());
            }
        }
    }
}

mod test {
    use crate::frontend::lexer::Lexer;

    use super::*;
    #[test]
    fn test() {
        let mut lexer = Lexer::new("(1 + foo) * 1");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        println!("{:?}", ast);
    }
    #[test]
    fn test_assignment_expression() {
        let mut lexer = Lexer::new("a = b + c");
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        println!("{:?}", ast);
    }
}
