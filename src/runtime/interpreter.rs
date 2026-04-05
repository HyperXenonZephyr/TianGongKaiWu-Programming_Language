use crate::ast::*;
use crate::runtime::value::{Environment, Value};
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

thread_local! {
    static RETURN_VALUE: RefCell<Option<Value>> = RefCell::new(None);
}

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut env = Environment::new();
        Self::add_native_functions(&mut env);
        Self { environment: env }
    }

    fn add_native_functions(env: &mut Environment) {
        // 使用标准库
        crate::runtime::stdlib::StandardLibrary::init(env);
    }

    pub fn interpret(&mut self, program: &Program) -> Result<Value, String> {
        let mut last_value = Value::Null;

        for statement in &program.statements {
            match self.execute_statement(statement) {
                Ok(value) => last_value = value,
                Err(e) => {
                    if e == "__return__" {
                        let val = RETURN_VALUE.with(|rv| rv.borrow_mut().take());
                        return Ok(val.unwrap_or(Value::Null));
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Ok(last_value)
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<Value, String> {
        match statement {
            Statement::VariableDecl(decl) => self.execute_variable_decl(decl),
            Statement::Assignment(assign) => self.execute_assignment(assign),
            Statement::Expression(expr) => self.evaluate_expression(expr),
            Statement::IfStatement(stmt) => self.execute_if_statement(stmt),
            Statement::LoopStatement(stmt) => self.execute_loop_statement(stmt),
            Statement::WhileStatement(stmt) => self.execute_while_statement(stmt),
            Statement::BreakStatement(_) => Err("break".to_string()), // 中断循环
            Statement::ReturnStatement(stmt) => self.execute_return_statement(stmt),
            Statement::FunctionDecl(decl) => self.execute_function_decl(decl),
            Statement::TryCatch(stmt) => self.execute_try_catch(stmt),
            Statement::PrintStatement(stmt) => self.execute_print_statement(stmt),
            Statement::ForEachStatement(stmt) => self.execute_foreach_statement(stmt),
            Statement::ImportStatement(_) => Ok(Value::Null), // 暂不实现
            Statement::ExportStatement(_) => Ok(Value::Null), // 暂不实现
        }
    }

    fn execute_variable_decl(&mut self, decl: &VariableDecl) -> Result<Value, String> {
        let value = if let Some(expr) = &decl.value {
            self.evaluate_expression(expr)?
        } else {
            Value::Null
        };

        self.environment.define(decl.name.clone(), value.clone());
        Ok(value)
    }

    fn execute_assignment(&mut self, assign: &Assignment) -> Result<Value, String> {
        let value = self.evaluate_expression(&assign.value)?;
        
        match &assign.target {
            AssignmentTarget::Identifier(name) => {
                self.environment.set(name.clone(), value.clone());
                Ok(value)
            }
            _ => Err("暫不支持成員賦值".to_string()),
        }
    }

    fn execute_if_statement(&mut self, stmt: &IfStatement) -> Result<Value, String> {
        let condition = self.evaluate_expression(&stmt.condition)?;
        
        if condition.is_truthy() {
            self.execute_block(&stmt.then_branch)
        } else {
            // 检查 else if 分支
            for else_if in &stmt.else_if_branches {
                let condition = self.evaluate_expression(&else_if.condition)?;
                if condition.is_truthy() {
                    return self.execute_block(&else_if.statements);
                }
            }
            
            // 执行 else 分支
            if let Some(else_branch) = &stmt.else_branch {
                self.execute_block(else_branch)
            } else {
                Ok(Value::Null)
            }
        }
    }

    fn execute_loop_statement(&mut self, stmt: &LoopStatement) -> Result<Value, String> {
        loop {
            match self.execute_block(&stmt.body) {
                Ok(_) => continue,
                Err(e) => {
                    if e == "break" {
                        break Ok(Value::Null);
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    fn execute_while_statement(&mut self, stmt: &WhileStatement) -> Result<Value, String> {
        while self.evaluate_expression(&stmt.condition)?.is_truthy() {
            match self.execute_block(&stmt.body) {
                Ok(_) => continue,
                Err(e) => {
                    if e == "break" {
                        break;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
        Ok(Value::Null)
    }

    fn execute_return_statement(&mut self, stmt: &ReturnStatement) -> Result<Value, String> {
        let value = if let Some(expr) = &stmt.value {
            self.evaluate_expression(expr)?
        } else {
            Value::Null
        };
        RETURN_VALUE.with(|rv| {
            *rv.borrow_mut() = Some(value);
        });
        Err("__return__".to_string())
    }

    fn execute_function_decl(&mut self, decl: &FunctionDecl) -> Result<Value, String> {
        // 先创建函数对象，但不添加到环境（避免循环引用）
        let func = crate::runtime::value::Function {
            name: decl.name.clone(),
            params: decl.params.iter().map(|p| p.name.clone()).collect(),
            body: Program {
                statements: decl.body.clone(),
                span: decl.span,
            },
            closure: self.environment.clone(),
        };

        let func_value = Value::Function(Rc::new(func));
        
        // 现在将函数添加到环境
        self.environment
            .define(decl.name.clone(), func_value);
        Ok(Value::Null)
    }

    fn execute_try_catch(&mut self, stmt: &TryCatch) -> Result<Value, String> {
        // 简化实现：只执行try块
        self.execute_block(&stmt.try_block)
    }

    fn execute_print_statement(&mut self, stmt: &PrintStatement) -> Result<Value, String> {
        for expr in &stmt.expressions {
            let value = self.evaluate_expression(expr)?;
            print!("{}", value);
        }
        println!();
        Ok(Value::Null)
    }

    fn execute_foreach_statement(&mut self, stmt: &ForEachStatement) -> Result<Value, String> {
        let iterable = self.evaluate_expression(&stmt.iterable)?;

        match iterable {
            Value::Array(arr) => {
                let mut last = Value::Null;
                for item in arr {
                    self.environment.define(stmt.variable.clone(), item);
                    match self.execute_block(&stmt.body) {
                        Ok(v) => last = v,
                        Err(e) if e == "break" => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(last)
            }
            Value::String(s) => {
                let mut last = Value::Null;
                for ch in s.chars() {
                    self.environment.define(stmt.variable.clone(), Value::String(ch.to_string()));
                    match self.execute_block(&stmt.body) {
                        Ok(v) => last = v,
                        Err(e) if e == "break" => break,
                        Err(e) => return Err(e),
                    }
                }
                Ok(last)
            }
            _ => Err("遍歷對象必須是數組或字符串".to_string()),
        }
    }

    fn execute_block(&mut self, statements: &[Statement]) -> Result<Value, String> {
        let env = Environment::new_with_parent(self.environment.clone());
        let mut interpreter = Interpreter {
            environment: env,
        };

        let mut last_value = Value::Null;
        for statement in statements {
            last_value = interpreter.execute_statement(statement)?;
        }

        // 更新父环境中的变量
        for (name, value) in interpreter.environment.variables {
            self.environment.set(name, value);
        }

        Ok(last_value)
    }

    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value, String> {
        match expr {
            Expression::Literal(lit) => self.evaluate_literal(lit),
            Expression::Identifier(name, _) => self.evaluate_identifier(name),
            Expression::Binary(bin) => self.evaluate_binary(bin),
            Expression::Unary(unary) => self.evaluate_unary(unary),
            Expression::Call(call) => self.evaluate_call(call),
            Expression::MemberAccess(access) => self.evaluate_member_access(access),
            Expression::IndexAccess(access) => self.evaluate_index_access(access),
            Expression::Array(arr) => self.evaluate_array(arr),
            Expression::Dict(dict) => self.evaluate_dict(dict),
            Expression::Parenthesized(expr, _) => self.evaluate_expression(expr),
        }
    }

    fn evaluate_literal(&self, lit: &Literal) -> Result<Value, String> {
        match lit {
            Literal::Number(n) => {
                // 尝试解析中文数字
                if n.chars().all(|c| c.is_ascii_digit() || c == '.') {
                    if n.contains('.') {
                        Ok(Value::Number(n.parse::<f64>().map_err(|e| e.to_string())?))
                    } else {
                        Ok(Value::Integer(n.parse::<i64>().map_err(|e| e.to_string())?))
                    }
                } else {
                    Ok(Self::chinese_to_number(n))
                }
            }
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Boolean(b) => Ok(Value::Boolean(*b)),
            Literal::Null => Ok(Value::Null),
        }
    }

    /// 中文数字转换（支持「點」小数）
    fn chinese_to_number(chinese: &str) -> Value {
        if chinese.contains('點') {
            let parts: Vec<&str> = chinese.splitn(2, '點').collect();
            let integer_part = Self::chinese_integer(parts[0]) as f64;
            let decimal_str = parts[1];
            let mut decimal_part = 0.0;
            let mut place = 0.1;
            for ch in decimal_str.chars() {
                let digit = match ch {
                    '零' => 0.0,
                    '一' => 1.0,
                    '二' => 2.0,
                    '三' => 3.0,
                    '四' => 4.0,
                    '五' => 5.0,
                    '六' => 6.0,
                    '七' => 7.0,
                    '八' => 8.0,
                    '九' => 9.0,
                    _ => continue,
                };
                decimal_part += digit * place;
                place *= 0.1;
            }
            Value::Number(integer_part + decimal_part)
        } else {
            Value::Integer(Self::chinese_integer(chinese))
        }
    }

    fn chinese_integer(chinese: &str) -> i64 {
        let mut result = 0;
        let mut temp = 0;

        for ch in chinese.chars() {
            match ch {
                '零' => continue,
                '一' => temp = 1,
                '二' | '兩' => temp = 2,
                '三' => temp = 3,
                '四' => temp = 4,
                '五' => temp = 5,
                '六' => temp = 6,
                '七' => temp = 7,
                '八' => temp = 8,
                '九' => temp = 9,
                '十' => {
                    if temp == 0 {
                        temp = 1;
                    }
                    result += temp * 10;
                    temp = 0;
                }
                '百' => {
                    if temp == 0 {
                        temp = 1;
                    }
                    result += temp * 100;
                    temp = 0;
                }
                '千' => {
                    if temp == 0 {
                        temp = 1;
                    }
                    result += temp * 1000;
                    temp = 0;
                }
                '萬' | '万' => {
                    if temp == 0 {
                        temp = 1;
                    }
                    result += temp;
                    result *= 10000;
                    temp = 0;
                }
                '億' | '亿' => {
                    if temp == 0 {
                        temp = 1;
                    }
                    result += temp;
                    result *= 100000000;
                    temp = 0;
                }
                _ => continue,
            }
        }

        result + temp
    }

    fn evaluate_identifier(&mut self, name: &str) -> Result<Value, String> {
        self.environment
            .get(name)
            .ok_or_else(|| format!("未定義的變量: {}", name))
    }

    fn evaluate_binary(&mut self, bin: &BinaryExpr) -> Result<Value, String> {
        let left = self.evaluate_expression(&bin.left)?;
        let right = self.evaluate_expression(&bin.right)?;

        match bin.op {
            BinaryOp::Add => self.binary_add(&left, &right),
            BinaryOp::Sub => self.binary_sub(&left, &right),
            BinaryOp::Mul => self.binary_mul(&left, &right),
            BinaryOp::Div => self.binary_div(&left, &right),
            BinaryOp::Mod => self.binary_mod(&left, &right),
            BinaryOp::Pow => self.binary_pow(&left, &right),
            BinaryOp::Equal => Ok(Value::Boolean(self.values_equal(&left, &right))),
            BinaryOp::NotEqual => Ok(Value::Boolean(!self.values_equal(&left, &right))),
            BinaryOp::Greater => self.binary_greater(&left, &right),
            BinaryOp::Less => self.binary_less(&left, &right),
            BinaryOp::GreaterEqual => self.binary_greater_equal(&left, &right),
            BinaryOp::LessEqual => self.binary_less_equal(&left, &right),
            BinaryOp::And => Ok(Value::Boolean(left.is_truthy() && right.is_truthy())),
            BinaryOp::Or => Ok(Value::Boolean(left.is_truthy() || right.is_truthy())),
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Integer(a), Value::Number(b)) => ((*a as f64) - b).abs() < f64::EPSILON,
            (Value::Number(a), Value::Integer(b)) => (a - (*b as f64)).abs() < f64::EPSILON,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }

    fn binary_add(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Number(*a as f64 + b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Number(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_string()))),
            (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_string(), b))),
            _ => Err("加法運算類型錯誤".to_string()),
        }
    }

    fn binary_sub(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Number(*a as f64 - b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Number(a - *b as f64)),
            _ => Err("減法運算類型錯誤".to_string()),
        }
    }

    fn binary_mul(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Number(*a as f64 * b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Number(a * *b as f64)),
            _ => Err("乘法運算類型錯誤".to_string()),
        }
    }

    fn binary_div(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Number(a / b)) }
            }
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Integer(a / b)) }
            }
            (Value::Integer(a), Value::Number(b)) => {
                if *b == 0.0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Number(*a as f64 / b)) }
            }
            (Value::Number(a), Value::Integer(b)) => {
                if *b == 0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Number(a / *b as f64)) }
            }
            _ => Err("除法運算類型錯誤".to_string()),
        }
    }

    fn binary_mod(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => {
                if *b == 0.0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Number(a % b)) }
            }
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Integer(a % b)) }
            }
            (Value::Integer(a), Value::Number(b)) => {
                if *b == 0.0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Number(*a as f64 % b)) }
            }
            (Value::Number(a), Value::Integer(b)) => {
                if *b == 0 { Err("除數不能為零".to_string()) }
                else { Ok(Value::Number(a % *b as f64)) }
            }
            _ => Err("取餘運算類型錯誤".to_string()),
        }
    }

    fn binary_pow(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.powf(*b))),
            (Value::Integer(a), Value::Integer(b)) => {
                if *b < 0 {
                    Ok(Value::Number((*a as f64).powf(*b as f64)))
                } else {
                    Ok(Value::Integer(a.pow(*b as u32)))
                }
            }
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Number((*a as f64).powf(*b))),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Number(a.powf(*b as f64))),
            _ => Err("冪運算類型錯誤".to_string()),
        }
    }

    fn binary_greater(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a > b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a > b)),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Boolean((*a as f64) > *b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Boolean(*a > (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a > b)),
            _ => Err("比較運算類型錯誤".to_string()),
        }
    }

    fn binary_less(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a < b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a < b)),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Boolean((*a as f64) < *b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Boolean(*a < (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a < b)),
            _ => Err("比較運算類型錯誤".to_string()),
        }
    }

    fn binary_greater_equal(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a >= b)),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Boolean((*a as f64) >= *b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Boolean(*a >= (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a >= b)),
            _ => Err("比較運算類型錯誤".to_string()),
        }
    }

    fn binary_less_equal(&self, left: &Value, right: &Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a <= b)),
            (Value::Integer(a), Value::Number(b)) => Ok(Value::Boolean((*a as f64) <= *b)),
            (Value::Number(a), Value::Integer(b)) => Ok(Value::Boolean(*a <= (*b as f64))),
            (Value::String(a), Value::String(b)) => Ok(Value::Boolean(a <= b)),
            _ => Err("比較運算類型錯誤".to_string()),
        }
    }

    fn evaluate_unary(&mut self, unary: &UnaryExpr) -> Result<Value, String> {
        let expr = self.evaluate_expression(&unary.expr)?;

        match unary.op {
            UnaryOp::Not => Ok(Value::Boolean(!expr.is_truthy())),
            UnaryOp::Neg => match expr {
                Value::Number(n) => Ok(Value::Number(-n)),
                Value::Integer(i) => Ok(Value::Integer(-i)),
                _ => Err("負號運算類型錯誤".to_string()),
            },
        }
    }

    fn evaluate_call(&mut self, call: &CallExpr) -> Result<Value, String> {
        let callee = self.evaluate_expression(&call.callee)?;
        let mut args = Vec::new();

        for arg in &call.args {
            args.push(self.evaluate_expression(arg)?);
        }

        match callee {
            Value::Function(func) => self.call_function(&func, args),
            Value::NativeFunction(func) => func(args),
            _ => Err("只能調用函數".to_string()),
        }
    }

    fn call_function(&mut self, func: &crate::runtime::value::Function, args: Vec<Value>) -> Result<Value, String> {
        if args.len() != func.params.len() {
            return Err(format!(
                "期望 {} 個參數，但得到 {} 個",
                func.params.len(),
                args.len()
            ));
        }

        // 创建新的环境
        let mut env = Environment::new_with_parent(func.closure.clone());
        
        // 绑定参数
        for (param, arg) in func.params.iter().zip(args.iter()) {
            env.define(param.clone(), arg.clone());
        }

        // 将函数本身绑定到环境中，支持递归调用
        env.define(func.name.clone(), Value::Function(Rc::new(func.clone())));

        // 执行函数体
        let mut interpreter = Interpreter { environment: env };
        interpreter.interpret(&func.body)
    }

    fn evaluate_member_access(&mut self, access: &MemberAccess) -> Result<Value, String> {
        let object = self.evaluate_expression(&access.object)?;
        
        match object {
            Value::Dict(dict) => {
                dict.get(&access.member)
                    .cloned()
                    .ok_or_else(|| format!("未找到成員: {}", access.member))
            }
            _ => Err("只能訪問字典的成員".to_string()),
        }
    }

    fn evaluate_index_access(&mut self, access: &IndexAccess) -> Result<Value, String> {
        let object = self.evaluate_expression(&access.object)?;
        let index = self.evaluate_expression(&access.index)?;
        
        match (object, index) {
            (Value::Array(arr), Value::Integer(i)) => {
                if i < 0 || i >= arr.len() as i64 {
                    Err("索引超出範圍".to_string())
                } else {
                    Ok(arr[i as usize].clone())
                }
            }
            (Value::Dict(dict), Value::String(key)) => {
                dict.get(&key)
                    .cloned()
                    .ok_or_else(|| format!("未找到鍵: {}", key))
            }
            _ => Err("索引訪問類型錯誤".to_string()),
        }
    }

    fn evaluate_array(&mut self, arr: &ArrayExpr) -> Result<Value, String> {
        let mut elements = Vec::new();
        
        for expr in &arr.elements {
            elements.push(self.evaluate_expression(expr)?);
        }
        
        Ok(Value::Array(elements))
    }

    fn evaluate_dict(&mut self, dict: &DictExpr) -> Result<Value, String> {
        let mut map = HashMap::new();
        
        for entry in &dict.entries {
            let key = match &entry.key {
                Expression::Literal(Literal::String(s)) => s.clone(),
                _ => return Err("字典鍵必須是字符串".to_string()),
            };
            
            let value = self.evaluate_expression(&entry.value)?;
            map.insert(key, value);
        }
        
        Ok(Value::Dict(map))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_interpret_variable_decl() {
        let source = "設 才 為 三";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        
        assert!(result.is_ok());
        assert_eq!(interpreter.environment.get("才"), Some(Value::Integer(3)));
    }

    #[test]
    fn test_interpret_arithmetic() {
        let source = "一 加 二";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Integer(3));
    }

    #[test]
    fn test_interpret_if_statement() {
        let source = "若 真 則 設 結果 為 一 終";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        
        assert!(result.is_ok());
        assert_eq!(interpreter.environment.get("結果"), Some(Value::Integer(1)));
    }

    #[test]
    fn test_interpret_function() {
        let source = "
            謂 加一(甲)
                返 甲 加 一
            終
            設 結果 為 加一(五)
        ";
        
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program);
        
        assert!(result.is_ok());
        assert_eq!(interpreter.environment.get("結果"), Some(Value::Integer(6)));
    }
}