use crate::ast::*;
use crate::lexer::{Span, Token, TokenInfo};

use std::collections::VecDeque;



pub struct Parser {
    tokens: VecDeque<TokenInfo>,
    previous_tokens: Vec<TokenInfo>, // 存储已消耗的token
}

impl Parser {
    pub fn new(tokens: Vec<TokenInfo>) -> Self {
        Self {
            tokens: VecDeque::from(tokens),
            previous_tokens: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        let start_span = if let Some(token) = self.tokens.front() {
            token.span
        } else {
            Span::new(0, 0, 1, 1)
        };

        while !self.is_at_end() {
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(e) => return Err(e),
            }
        }

        let end_span = if let Some(token) = self.tokens.back() {
            token.span
        } else {
            Span::new(0, 0, 1, 1)
        };

        Ok(Program {
            statements,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        // 检查下一个token
        if let Some(token_info) = self.peek() {
            match &token_info.token {
                Token::Set => {
                    self.advance(); // 消耗Set
                    self.parse_variable_decl()
                }
                Token::Var => {
                    // 通假字「才」：检查下一个token
                    // 如果是标识符，再下一个是「為」，则是变量声明；否则是表达式语句
                    if let Some(next_token) = self.peek_nth(1) {
                        if matches!(&next_token.token, Token::Identifier(_)) {
                            // 检查下下个token是否为「為」
                            if let Some(next_next_token) = self.peek_nth(2) {
                                if matches!(&next_next_token.token, Token::Assign) {
                                    self.advance(); // 消耗Var
                                    self.parse_variable_decl()
                                } else {
                                    self.parse_assignment_statement()
                                }
                            } else {
                                self.parse_assignment_statement()
                            }
                        } else {
                            self.parse_assignment_statement()
                        }
                    } else {
                        self.parse_assignment_statement()
                    }
                }
                Token::If => {
                    self.advance(); // 消耗If
                    self.parse_if_statement()
                }
                Token::Loop => {
                    self.advance(); // 消耗Loop
                    self.parse_loop_statement()
                }
                Token::While => {
                    self.advance(); // 消耗While
                    self.parse_while_statement()
                }
                Token::Break => {
                    self.advance(); // 消耗Break
                    self.parse_break_statement()
                }
                Token::Return => {
                    self.advance(); // 消耗Return
                    self.parse_return_statement()
                }
                Token::Func => {
                    self.advance(); // 消耗Func
                    self.parse_function_decl()
                }
                Token::Try => {
                    self.advance(); // 消耗Try
                    self.parse_try_catch()
                }
                Token::Call => {
                    self.advance(); // 消耗Call（執）
                    // 解析为表达式语句 - 下一个应为函数调用
                    let expr = self.parse_expression()?;
                    // 检查倒装输出：執 func() 曰
                    if self.match_token(Token::Print) {
                        let expr_span = expr.span();
                        Ok(Statement::PrintStatement(PrintStatement {
                            expressions: vec![expr],
                            span: Span::new(
                                expr_span.start,
                                self.previous().span.end,
                                expr_span.line,
                                expr_span.column,
                            ),
                        }))
                    } else {
                        Ok(Statement::Expression(expr))
                    }
                }
                Token::ForEach => {
                    self.advance(); // 消耗ForEach（遍）
                    self.parse_foreach_statement()
                }
                Token::Print => {
                    self.advance(); // 消耗Print
                    self.parse_print_statement()
                }
                Token::Import => {
                    self.advance(); // 消耗Import
                    self.parse_import_statement()
                }
                Token::Export => {
                    self.advance(); // 消耗Export
                    self.parse_export_statement()
                }
                _ => self.parse_assignment_statement()
            }
        } else {
            Err("期望語句".to_string())
        }
    }

    fn parse_assignment_statement(&mut self) -> Result<Statement, String> {
        // 解析表达式
        let expr = self.parse_expression()?;
        
        // 检查是否为倒装句：表达式 曰
        if self.match_token(Token::Print) {
            // 这是倒装输出语句：表达式 曰
            let expr_span = expr.span();
            return Ok(Statement::PrintStatement(PrintStatement {
                expressions: vec![expr],
                span: Span::new(
                    expr_span.start,
                    self.previous().span.end,
                    expr_span.line,
                    expr_span.column,
                ),
            }));
        }
        
        // 检查是否为标识符 為 表达式（省略設关键字的省略句）
        if let Expression::Identifier(ref name, ident_span) = expr {
            if self.match_token(Token::Assign) {
                let value = self.parse_expression()?;
                let end_span = value.span();
                // 创建变量声明（省略設关键字的省略句）
                return Ok(Statement::VariableDecl(VariableDecl {
                    name: name.clone(),
                    value: Some(value),
                    span: Span::new(ident_span.start, end_span.end, ident_span.line, ident_span.column),
                }));
            }
        }
        
        // 否则作为普通表达式语句
        Ok(Statement::Expression(expr))
    }

    fn parse_variable_decl(&mut self) -> Result<Statement, String> {
        // 注意：self.previous() 现在返回正确的上一个token
        let start_span = self.previous().span;

        // 解析变量名（可以是标识符或通假字"才"）
        let name = if let Some(token_info) = self.advance() {
            match token_info.token {
                Token::Identifier(name) => name,
                Token::Var => "才".to_string(), // 通假字"才"作为变量名
                token => return Err(format!("期望變量名，得到：{:?}", token)),
            }
        } else {
            return Err("期望變量名（沒有更多token）".to_string());
        };

        // 解析赋值符号
        self.consume(Token::Assign, "期望「為」")?;

        // 解析表达式
        let value = self.parse_expression()?;
        let end_span = value.span();

        Ok(Statement::VariableDecl(VariableDecl {
            name,
            value: Some(value),
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_if_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        // 解析条件
        let condition = self.parse_expression()?;

        // 解析then分支
        self.consume(Token::Then, "期望「則」")?;

        let mut then_branch = Vec::new();
        while !self.check(Token::ElseIf) && !self.check(Token::Else) && !self.check(Token::End) {
            then_branch.push(self.parse_statement()?);
        }

        // 解析else if分支
        let mut else_if_branches = Vec::new();
        while self.match_token(Token::ElseIf) {
            let condition = self.parse_expression()?;
            self.consume(Token::Then, "期望「則」")?;

            let mut statements = Vec::new();
            while !self.check(Token::ElseIf) && !self.check(Token::Else) && !self.check(Token::End) {
                statements.push(self.parse_statement()?);
            }

            else_if_branches.push(ElseIfBranch {
                condition,
                statements,
                span: self.previous().span,
            });
        }

        // 解析else分支
        let mut else_branch = None;
        if self.match_token(Token::Else) {
            let mut statements = Vec::new();
            while !self.check(Token::End) {
                statements.push(self.parse_statement()?);
            }
            else_branch = Some(statements);
        }

        // 解析end
        self.consume(Token::End, "期望「終」")?;

        let end_span = self.previous().span;

        Ok(Statement::IfStatement(IfStatement {
            condition,
            then_branch,
            else_if_branches,
            else_branch,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_loop_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        // 解析计数表达式或循环条件
        let count_expr = self.parse_expression()?;

        // 检查是否是计次循环：走 N 次 ... 終
        if self.match_token(Token::Times) {
            let counter = format!("__迴圈計數_{}", start_span.start);
            let mut body_stmts = Vec::new();
            while !self.check(Token::End) {
                body_stmts.push(self.parse_statement()?);
            }
            self.consume(Token::End, "期望「終」")?;
            let end_span = self.previous().span;

            // 构造：设 counter 为 0; 循 counter < N { body; counter = counter + 1 }
            let mut stmts = Vec::new();

            // 设 counter 为 0
            stmts.push(Statement::VariableDecl(VariableDecl {
                name: counter.clone(),
                value: Some(Expression::Literal(Literal::Number("0".to_string()))),
                span: start_span,
            }));

            // 循 counter < N { body ... counter = counter + 1 }
            let mut loop_body = body_stmts;
            loop_body.push(Statement::Assignment(Assignment {
                target: AssignmentTarget::Identifier(counter.clone()),
                value: Expression::Binary(BinaryExpr {
                    left: Box::new(Expression::Identifier(counter.clone(), start_span)),
                    op: BinaryOp::Add,
                    right: Box::new(Expression::Literal(Literal::Number("1".to_string()))),
                    span: start_span,
                }),
                span: start_span,
            }));

            stmts.push(Statement::WhileStatement(WhileStatement {
                condition: Expression::Binary(BinaryExpr {
                    left: Box::new(Expression::Identifier(counter.clone(), start_span)),
                    op: BinaryOp::Less,
                    right: Box::new(count_expr),
                    span: start_span,
                }),
                body: loop_body,
                span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
            }));

            // 包装为 if(true) 以返回单个语句
            return Ok(Statement::IfStatement(IfStatement {
                condition: Expression::Literal(Literal::Boolean(true)),
                then_branch: stmts,
                else_if_branches: vec![],
                else_branch: None,
                span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
            }));
        }

        // 否则按 while 循环处理：走 condition ... 終
        let mut body = Vec::new();
        while !self.check(Token::End) && !self.check(Token::Break) {
            body.push(self.parse_statement()?);
        }

        // 检查是否有Break语句
        if self.match_token(Token::Break) {
            body.push(Statement::BreakStatement(BreakStatement {
                span: self.previous().span,
            }));
        }

        self.consume(Token::End, "期望「終」")?;
        let end_span = self.previous().span;

        // 创建WhileStatement而不是LoopStatement
        Ok(Statement::WhileStatement(WhileStatement {
            condition: count_expr,
            body,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_while_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        let condition = self.parse_expression()?;

        let mut body = Vec::new();
        while !self.check(Token::End) && !self.check(Token::Break) {
            body.push(self.parse_statement()?);
        }

        // 检查是否有Break语句
        if self.match_token(Token::Break) {
            body.push(Statement::BreakStatement(BreakStatement {
                span: self.previous().span,
            }));
        }

        self.consume(Token::End, "期望「終」")?;
        let end_span = self.previous().span;

        Ok(Statement::WhileStatement(WhileStatement {
            condition,
            body,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_break_statement(&mut self) -> Result<Statement, String> {
        let span = self.previous().span;
        Ok(Statement::BreakStatement(BreakStatement { span }))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        let value = if !self.check(Token::Semicolon) && !self.is_at_end() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let end_span = if let Some(expr) = &value {
            expr.span()
        } else {
            start_span
        };

        Ok(Statement::ReturnStatement(ReturnStatement {
            value,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_function_decl(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        // 解析函数名
        let name = if let Token::Identifier(name) = &self.consume(Token::Identifier("".to_string()), "期望函數名")?.token {
            name.clone()
        } else {
            return Err("期望函數名".to_string());
        };

        // 解析参数列表
        self.consume(Token::LParen, "期望「(」")?;
        let mut params = Vec::new();

        if !self.check(Token::RParen) {
            loop {
                let param_name = if let Token::Identifier(name) = &self.consume(Token::Identifier("".to_string()), "期望參數名")?.token {
                    name.clone()
                } else {
                    return Err("期望參數名".to_string());
                };

                let param_type = if self.match_token(Token::Colon) {
                    if let Token::Identifier(type_name) = &self.consume(Token::Identifier("".to_string()), "期望類型名")?.token {
                        Some(type_name.clone())
                    } else {
                        return Err("期望類型名".to_string());
                    }
                } else {
                    None
                };

                params.push(Param {
                    name: param_name,
                    param_type,
                    span: self.previous().span,
                });

                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }

        self.consume(Token::RParen, "期望「)」")?;

        // 解析返回类型
        let return_type = if self.match_token(Token::Colon) {
            if let Token::Identifier(type_name) = &self.consume(Token::Identifier("".to_string()), "期望返回類型")?.token {
                Some(type_name.clone())
            } else {
                return Err("期望返回類型".to_string());
            }
        } else {
            None
        };

        // 解析函数体
        let mut body = Vec::new();
        while !self.check(Token::End) {
            body.push(self.parse_statement()?);
        }

        self.consume(Token::End, "期望「終」")?;
        let end_span = self.previous().span;

        Ok(Statement::FunctionDecl(FunctionDecl {
            name,
            params,
            body,
            return_type,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_try_catch(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        // 解析try块
        let mut try_block = Vec::new();
        while !self.check(Token::Catch) && !self.check(Token::End) {
            try_block.push(self.parse_statement()?);
        }

        // 解析catch块
        let mut catch_blocks = Vec::new();
        while self.match_token(Token::Catch) {
            let exception_name = if let Token::Identifier(name) = &self.consume(Token::Identifier("".to_string()), "期望異常名")?.token {
                name.clone()
            } else {
                return Err("期望異常名".to_string());
            };

            let exception_type = if self.match_token(Token::Colon) {
                if let Token::Identifier(type_name) = &self.consume(Token::Identifier("".to_string()), "期望異常類型")?.token {
                    Some(type_name.clone())
                } else {
                    return Err("期望異常類型".to_string());
                }
            } else {
                None
            };

            let mut statements = Vec::new();
            while !self.check(Token::Catch) && !self.check(Token::Finally) && !self.check(Token::End) {
                statements.push(self.parse_statement()?);
            }

            catch_blocks.push(CatchBlock {
                exception_type,
                exception_name,
                statements,
                span: self.previous().span,
            });
        }

        // 解析finally块
        let mut finally_block = None;
        if self.match_token(Token::Finally) {
            let mut statements = Vec::new();
            while !self.check(Token::End) {
                statements.push(self.parse_statement()?);
            }
            finally_block = Some(statements);
        }

        self.consume(Token::End, "期望「終」")?;
        let end_span = self.previous().span;

        Ok(Statement::TryCatch(TryCatch {
            try_block,
            catch_blocks,
            finally_block,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_print_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        let mut expressions = Vec::new();

        // 支持函数调用风格：輸出(expr1, expr2, ...)
        if self.check(Token::LParen) {
            self.advance(); // consume (
            if !self.check(Token::RParen) {
                loop {
                    expressions.push(self.parse_expression()?);
                    if !self.match_token(Token::Comma) {
                        break;
                    }
                }
            }
            self.consume(Token::RParen, "期望「)」")?;
        } else if !self.check(Token::Semicolon) && !self.is_at_end() && !self.check_next_is_statement_end() {
            loop {
                expressions.push(self.parse_expression()?);

                if self.is_at_end() || self.check_next_is_statement_end() {
                    break;
                }
            }
        }

        let end_span = if let Some(last_expr) = expressions.last() {
            last_expr.span()
        } else {
            start_span
        };

        Ok(Statement::PrintStatement(PrintStatement {
            expressions,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_import_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        let module = if let Token::StringLiteral(module) = &self.consume(Token::StringLiteral("".to_string()), "期望模塊名")?.token {
            module.clone()
        } else {
            return Err("期望模塊名".to_string());
        };

        let mut imports = Vec::new();
        if self.match_token(Token::Import) {
            if self.match_token(Token::Mul) {
                imports.push(ImportItem::All);
            } else {
                loop {
                    let item = if let Token::Identifier(name) = &self.consume(Token::Identifier("".to_string()), "期望導入項")?.token {
                        name.clone()
                    } else {
                        return Err("期望導入項".to_string());
                    };

                    if self.match_token(Token::As) {
                        let alias = if let Token::Identifier(alias) = &self.consume(Token::Identifier("".to_string()), "期望別名")?.token {
                            alias.clone()
                        } else {
                            return Err("期望別名".to_string());
                        };
                        imports.push(ImportItem::Alias(item, alias));
                    } else {
                        imports.push(ImportItem::Specific(item));
                    }

                    if !self.match_token(Token::Comma) {
                        break;
                    }
                }
            }
        }

        let end_span = self.previous().span;

        Ok(Statement::ImportStatement(ImportStatement {
            module,
            imports,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_export_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        let mut exports = Vec::new();
        if self.match_token(Token::Mul) {
            exports.push(ExportItem::All);
        } else {
            loop {
                let export = if let Token::Identifier(name) = &self.consume(Token::Identifier("".to_string()), "期望導出項")?.token {
                    name.clone()
                } else {
                    return Err("期望導出項".to_string());
                };

                // 判断是函数还是变量
                if self.peek().map(|t| matches!(t.token, Token::LParen)).unwrap_or(false) {
                    exports.push(ExportItem::Function(export));
                } else {
                    exports.push(ExportItem::Variable(export));
                }

                if !self.match_token(Token::Comma) {
                    break;
                }
            }
        }

        let end_span = self.previous().span;

        Ok(Statement::ExportStatement(ExportStatement {
            exports,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }



    fn parse_foreach_statement(&mut self) -> Result<Statement, String> {
        let start_span = self.previous().span;

        // 遍 變量 於 集合 ... 終
        let var_name = if let Token::Identifier(name) = &self.consume(Token::Identifier("".to_string()), "期望變量名")?.token {
            name.clone()
        } else {
            return Err("期望變量名".to_string());
        };

        // 消耗「於」
        self.consume(Token::In, "期望「於」")?;

        let iterable = self.parse_expression()?;

        let mut body = Vec::new();
        while !self.check(Token::End) {
            body.push(self.parse_statement()?);
        }
        self.consume(Token::End, "期望「終」")?;
        let end_span = self.previous().span;

        Ok(Statement::ForEachStatement(ForEachStatement {
            variable: var_name,
            iterable,
            body,
            span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
        }))
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, String> {
        // 在文言文编程语言中，赋值操作在语句级别处理，不在表达式级别
        // 所以这里只解析表达式，不处理赋值
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_and()?;

        while self.match_token(Token::Or) {
            let op = BinaryOp::Or;
            let right = self.parse_and()?;
            let span = Span::new(
                expr.span().start,
                right.span().end,
                expr.span().line,
                expr.span().column,
            );
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_equality()?;

        while self.match_token(Token::And) {
            let op = BinaryOp::And;
            let right = self.parse_equality()?;
            let span = Span::new(
                expr.span().start,
                right.span().end,
                expr.span().line,
                expr.span().column,
            );
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = if self.match_token(Token::Equal) {
            Some(BinaryOp::Equal)
        } else if self.match_token(Token::NotEqual) {
            Some(BinaryOp::NotEqual)
        } else {
            None
        } {
            let right = self.parse_comparison()?;
            let span = Span::new(
                expr.span().start,
                right.span().end,
                expr.span().line,
                expr.span().column,
            );
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_term()?;

        while let Some(op) = if self.match_token(Token::Greater) {
            Some(BinaryOp::Greater)
        } else if self.match_token(Token::GreaterEqual) {
            Some(BinaryOp::GreaterEqual)
        } else if self.match_token(Token::Less) {
            Some(BinaryOp::Less)
        } else if self.match_token(Token::LessEqual) {
            Some(BinaryOp::LessEqual)
        } else {
            None
        } {
            let right = self.parse_term()?;
            let span = Span::new(
                expr.span().start,
                right.span().end,
                expr.span().line,
                expr.span().column,
            );
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_factor()?;

        while let Some(op) = if self.match_token(Token::Add) {
            Some(BinaryOp::Add)
        } else if self.match_token(Token::Sub) {
            Some(BinaryOp::Sub)
        } else {
            None
        } {
            let right = self.parse_factor()?;
            let span = Span::new(
                expr.span().start,
                right.span().end,
                expr.span().line,
                expr.span().column,
            );
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = if self.match_token(Token::Mul) {
            Some(BinaryOp::Mul)
        } else if self.match_token(Token::Div) {
            Some(BinaryOp::Div)
        } else if self.match_token(Token::Mod) {
            Some(BinaryOp::Mod)
        } else if self.match_token(Token::Pow) {
            Some(BinaryOp::Pow)
        } else {
            None
        } {
            let right = self.parse_unary()?;
            let span = Span::new(
                expr.span().start,
                right.span().end,
                expr.span().line,
                expr.span().column,
            );
            expr = Expression::Binary(BinaryExpr {
                left: Box::new(expr),
                op,
                right: Box::new(right),
                span,
            });
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, String> {
        if self.match_token(Token::Not) {
            let op = UnaryOp::Not;
            let right = self.parse_unary()?;
            let span = right.span();
            return Ok(Expression::Unary(UnaryExpr {
                op,
                expr: Box::new(right),
                span,
            }));
        }

        if self.match_token(Token::Sub) {
            let op = UnaryOp::Neg;
            let right = self.parse_unary()?;
            let span = right.span();
            return Ok(Expression::Unary(UnaryExpr {
                op,
                expr: Box::new(right),
                span,
            }));
        }

        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expression, String> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(Token::LParen) {
                let mut args = Vec::new();
                if !self.check(Token::RParen) {
                    loop {
                        args.push(self.parse_expression()?);
                        if !self.match_token(Token::Comma) {
                            break;
                        }
                    }
                }
                self.consume(Token::RParen, "期望「)」")?;
                
                let span = Span::new(
                    expr.span().start,
                    self.previous().span.end,
                    expr.span().line,
                    expr.span().column,
                );
                expr = Expression::Call(CallExpr {
                    callee: Box::new(expr),
                    args,
                    span,
                });
            } else if self.match_token(Token::Dot) {
                let member = if let Token::Identifier(name) = &self.consume(Token::Identifier("".to_string()), "期望成員名")?.token {
                    name.clone()
                } else {
                    return Err("期望成員名".to_string());
                };
                
                let span = Span::new(
                    expr.span().start,
                    self.previous().span.end,
                    expr.span().line,
                    expr.span().column,
                );
                expr = Expression::MemberAccess(MemberAccess {
                    object: Box::new(expr),
                    member,
                    span,
                });
            } else if self.match_token(Token::LBracket) {
                let index = self.parse_expression()?;
                self.consume(Token::RBracket, "期望「]」")?;
                
                let span = Span::new(
                    expr.span().start,
                    self.previous().span.end,
                    expr.span().line,
                    expr.span().column,
                );
                expr = Expression::IndexAccess(IndexAccess {
                    object: Box::new(expr),
                    index: Box::new(index),
                    span,
                });
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        if self.match_token(Token::True) {
            let _span = self.previous().span;
            return Ok(Expression::Literal(Literal::Boolean(true)));
        }

        if self.match_token(Token::False) {
            let _span = self.previous().span;
            return Ok(Expression::Literal(Literal::Boolean(false)));
        }

        if self.match_token(Token::Null) {
            let _span = self.previous().span;
            return Ok(Expression::Literal(Literal::Null));
        }

        if let Some(token_info) = self.peek() {
            match &token_info.token {
                Token::Number(_) => {
                    let token = self.advance().unwrap();
                    if let Token::Number(n) = token.token {
                        return Ok(Expression::Literal(Literal::Number(n)));
                    }
                }
                Token::StringLiteral(_) => {
                    let token = self.advance().unwrap();
                    if let Token::StringLiteral(s) = token.token {
                        return Ok(Expression::Literal(Literal::String(s)));
                    }
                }
                Token::Identifier(_) => {
                    let token = self.advance().unwrap();
                    if let Token::Identifier(name) = token.token {
                        return Ok(Expression::Identifier(name, token.span));
                    }
                }
                Token::Var => {
                    let token = self.advance().unwrap();
                    // 通假字「才」作为标识符
                    return Ok(Expression::Identifier("才".to_string(), token.span));
                }
                Token::Call => {
                    // 「執」在表达式中作为函数调用前缀，直接透传解析
                    self.advance();
                    return self.parse_call();
                }
                _ => {}
            }
        }

        if self.match_token(Token::LParen) {
            let expr = self.parse_expression()?;
            self.consume(Token::RParen, "期望「)」")?;
            let span = Span::new(
                self.previous().span.start - 1,
                self.previous().span.end,
                self.previous().span.line,
                self.previous().span.column - 1,
            );
            return Ok(Expression::Parenthesized(Box::new(expr), span));
        }

        if self.match_token(Token::LBracket) {
            let start_span = self.previous().span;
            let mut elements = Vec::new();

            if !self.check(Token::RBracket) {
                loop {
                    elements.push(self.parse_expression()?);
                    if !self.match_token(Token::Comma) {
                        break;
                    }
                }
            }

            self.consume(Token::RBracket, "期望「]」")?;
            let end_span = self.previous().span;

            return Ok(Expression::Array(ArrayExpr {
                elements,
                span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
            }));
        }

        if self.match_token(Token::LBrace) {
            let start_span = self.previous().span;
            let mut entries = Vec::new();

            if !self.check(Token::RBrace) {
                loop {
                    let key = self.parse_expression()?;
                    self.consume(Token::Colon, "期望「:」")?;
                    let value = self.parse_expression()?;
                    
                    let entry_span = Span::new(
                        key.span().start,
                        value.span().end,
                        key.span().line,
                        key.span().column,
                    );
                    entries.push(DictEntry {
                        key,
                        value,
                        span: entry_span,
                    });

                    if !self.match_token(Token::Comma) {
                        break;
                    }
                }
            }

            self.consume(Token::RBrace, "期望「}」")?;
            let end_span = self.previous().span;

            return Ok(Expression::Dict(DictExpr {
                entries,
                span: Span::new(start_span.start, end_span.end, start_span.line, start_span.column),
            }));
        }

        Err("期望表達式".to_string())
    }

    // 辅助方法
    fn match_token(&mut self, token: Token) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token: Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek()
                .map(|t| std::mem::discriminant(&t.token) == std::mem::discriminant(&token))
                .unwrap_or(false)
        }
    }

    fn check_next_is_statement_end(&self) -> bool {
        if self.is_at_end() {
            return true;
        }
        
        // 检查下一个token是否是语句结束的标志
        // 语句结束的标志包括：换行、分号、end关键字等
        // 对于文言文编程语言，我们主要检查是否是关键字或语句开始
        if let Some(token_info) = self.peek() {
            match &token_info.token {
                Token::Set | Token::Var | Token::If | Token::Loop | Token::While |
                Token::Break | Token::Return | Token::Func | Token::Try | Token::Print |
                Token::Import | Token::Export | Token::End | Token::Else | Token::ElseIf |
                Token::Then | Token::Catch | Token::Except | Token::Finally |
                Token::Call | Token::ForEach => true,
                _ => false,
            }
        } else {
            true
        }
    }

    fn advance(&mut self) -> Option<TokenInfo> {
        if !self.is_at_end() {
            if let Some(token) = self.tokens.pop_front() {
                self.previous_tokens.push(token.clone());
                Some(token)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn is_at_end(&self) -> bool {
        self.tokens.is_empty()
    }

    fn peek(&self) -> Option<&TokenInfo> {
        self.tokens.front()
    }

    fn peek_nth(&self, n: usize) -> Option<&TokenInfo> {
        self.tokens.iter().nth(n)
    }

    fn previous(&self) -> TokenInfo {
        // 返回最后一个消耗的token
        self.previous_tokens
            .last()
            .cloned()
            .unwrap_or_else(|| TokenInfo {
                token: Token::Error,
                span: Span::new(0, 0, 0, 0),
            })
    }

    fn consume(&mut self, token: Token, message: &str) -> Result<TokenInfo, String> {
        if self.check(token) {
            Ok(self.advance().unwrap())
        } else {
            Err(message.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_variable_decl() {
        let source = "設 才 為 三";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse();
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::VariableDecl(decl) = &program.statements[0] {
            assert_eq!(decl.name, "才");
            assert!(decl.value.is_some());
        } else {
            panic!("期望 VariableDecl");
        }
    }

    #[test]
    fn test_parse_expression() {
        let source = "一 加 二";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse();
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::Expression(expr) = &program.statements[0] {
            if let Expression::Binary(bin) = expr {
                assert!(matches!(bin.op, BinaryOp::Add));
            } else {
                panic!("期望 Binary 表達式");
            }
        } else {
            panic!("期望 Expression 語句");
        }
    }

    #[test]
    fn test_parse_if_statement() {
        let source = "若 真 則 設 才 為 三 終";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        
        let result = parser.parse();
        assert!(result.is_ok());
        
        let program = result.unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Statement::IfStatement(if_stmt) = &program.statements[0] {
            assert!(matches!(if_stmt.condition, Expression::Literal(Literal::Boolean(true))));
            assert_eq!(if_stmt.then_branch.len(), 1);
        } else {
            panic!("期望 IfStatement");
        }
    }
}