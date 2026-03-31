use logos::Logos;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // 关键字
    #[token("設")]
    Set,
    #[token("為")]
    Assign,
    #[token("才")]
    Var,
    #[token("若")]
    If,
    #[token("若否")]
    Else,
    #[token("若然則")]
    ElseIf,
    #[token("則")]
    Then,
    #[token("走")]
    Loop,
    #[token("循")]
    While,
    #[token("止")]
    Break,
    #[token("返")]
    Return,
    #[token("歸")]
    Yield,
    #[token("謂")]
    Func,
    #[token("執")]
    Call,
    #[token("試")]
    Try,
    #[token("捕")]
    Catch,
    #[token("說")]
    Except,
    #[token("拋")]
    Throw,
    #[token("發")]
    Raise,
    #[token("finally")]
    Finally,
    #[token("as")]
    As,
    #[token("讀")]
    Read,
    #[token("寫")]
    Write,
    #[token("存")]
    Save,
    #[token("刪")]
    Delete,
    #[token("輸出")]
    #[token("曰")]
    Print,
    #[token("輸入")]
    Input,
    #[token("或")]
    Or,
    #[token("且")]
    And,
    #[token("非")]
    Not,
    #[token("真")]
    True,
    #[token("假")]
    False,
    #[token("無")]
    Null,
    #[token("引")]
    Import,
    #[token("納")]
    Export,
    #[token("記")]
    Log,
    #[token("錄")]
    Record,
    #[token("終")]
    End,

    // 运算符
    #[token("加")]
    Add,
    #[token("減")]
    Sub,
    #[token("乘")]
    Mul,
    #[token("除")]
    Div,
    #[token("餘")]
    Mod,
    #[token("冪")]
    Pow,
    #[token("等於")]
    Equal,
    #[token("*")]
    MulToken,
    #[token("不等於")]
    NotEqual,
    #[token("大於")]
    Greater,
    #[token("小於")]
    Less,
    #[token("大於等於")]
    GreaterEqual,
    #[token("小於等於")]
    LessEqual,
    #[token("存之")]
    Store,

    // 标点符号
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(",")]
    #[token("，")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Dot,


    // 字面量
    #[regex(r"[零一二三四五六七八九十百千萬億兆]+", |lex| lex.slice().to_string(), priority = 3)]
    #[regex(r"[0-9]+(?:\.[0-9]+)?", |lex| lex.slice().to_string(), priority = 3)]
    Number(String),

    #[regex(r#"「[^」]*」"#, |lex| {
        let s = lex.slice();
        // 跳过第一个和最后一个字符（中文引号）
        let chars: Vec<char> = s.chars().collect();
        if chars.len() >= 2 {
            chars[1..chars.len()-1].iter().collect()
        } else {
            String::new()
        }
    })]
    #[regex(r#""[^"]*""#, |lex| {
        let s = lex.slice();
        // 跳过第一个和最后一个字符（英文引号）
        let chars: Vec<char> = s.chars().collect();
        if chars.len() >= 2 {
            chars[1..chars.len()-1].iter().collect()
        } else {
            String::new()
        }
    })]
    StringLiteral(String),

    // 标识符（繁体中文）
    #[regex(r"[\p{Unified_Ideograph}][\p{Unified_Ideograph}\p{Nd}_]*", |lex| lex.slice().to_string(), priority = 2)]
    Identifier(String),

    // 空白字符
    #[regex(r"[ \t\n\r]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip, priority = 4)]
    #[regex(r"注[:：][^\n]*", logos::skip, priority = 4)]
    Whitespace,

    // 错误
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Set => write!(f, "設"),
            Token::Assign => write!(f, "曰"),
            Token::Var => write!(f, "才"),
            Token::If => write!(f, "若"),
            Token::Else => write!(f, "若否"),
            Token::ElseIf => write!(f, "若然則"),
            Token::Then => write!(f, "則"),
            Token::Loop => write!(f, "走"),
            Token::While => write!(f, "循"),
            Token::Break => write!(f, "止"),
            Token::Return => write!(f, "返"),
            Token::Yield => write!(f, "歸"),
            Token::Func => write!(f, "謂"),
            Token::Call => write!(f, "執"),
            Token::Try => write!(f, "試"),
            Token::Catch => write!(f, "捕"),
            Token::Except => write!(f, "說"),
            Token::Throw => write!(f, "拋"),
            Token::Raise => write!(f, "發"),
            Token::Finally => write!(f, "finally"),
            Token::As => write!(f, "as"),
            Token::Read => write!(f, "讀"),
            Token::Write => write!(f, "寫"),
            Token::Save => write!(f, "存"),
            Token::Delete => write!(f, "刪"),
            Token::Print => write!(f, "輸出"),
            Token::Input => write!(f, "輸入"),
            Token::Or => write!(f, "或"),
            Token::And => write!(f, "且"),
            Token::Not => write!(f, "非"),
            Token::True => write!(f, "真"),
            Token::False => write!(f, "假"),
            Token::Null => write!(f, "無"),
            Token::Import => write!(f, "引"),
            Token::Export => write!(f, "納"),
            Token::Log => write!(f, "記"),
            Token::Record => write!(f, "錄"),
            Token::End => write!(f, "終"),
            Token::Add => write!(f, "加"),
            Token::Sub => write!(f, "減"),
            Token::Mul => write!(f, "乘"),
            Token::Div => write!(f, "除"),
            Token::Mod => write!(f, "餘"),
            Token::Pow => write!(f, "冪"),
            Token::Equal => write!(f, "等於"),
            Token::MulToken => write!(f, "*"),
            Token::NotEqual => write!(f, "不等於"),
            Token::Greater => write!(f, "大於"),
            Token::Less => write!(f, "小於"),
            Token::GreaterEqual => write!(f, "大於等於"),
            Token::LessEqual => write!(f, "小於等於"),
            Token::Store => write!(f, "存之"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LBrace => write!(f, "{{"),
            Token::RBrace => write!(f, "}}"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Semicolon => write!(f, ";"),
            Token::Dot => write!(f, "."),
            Token::Number(n) => write!(f, "數字({})", n),
            Token::StringLiteral(s) => write!(f, "字符串「{}」", s),
            Token::Identifier(id) => write!(f, "標識符({})", id),
            Token::Whitespace => write!(f, "空白"),
            Token::Error => write!(f, "錯誤"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

pub struct Lexer<'a> {
    source: &'a str,
    inner: logos::Lexer<'a, Token>,
    current_line: usize,
    current_column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            inner: Token::lexer(source),
            current_line: 1,
            current_column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<TokenInfo> {
        let mut tokens = Vec::new();
        
        while let Some(result) = self.inner.next() {
            match result {
                Ok(token) => {
                    let span = self.inner.span();
                    let start = span.start;
                    let end = span.end;
                    
                    // 计算行和列
                    let (line, column) = self.calculate_position(start);
                    
                    tokens.push(TokenInfo {
                        token,
                        span: Span::new(start, end, line, column),
                    });
                    
                    // 更新当前位置
                    self.update_position(start, end);
                }
                Err(_) => {
                    // 跳过错误token
                    continue;
                }
            }
        }
        
        tokens
    }

    fn calculate_position(&self, pos: usize) -> (usize, usize) {
        let mut line = 1;
        let mut column = 1;
        
        // 使用字符迭代器，但需要跟踪字节位置
        let mut byte_pos = 0;
        for ch in self.source.chars() {
            if byte_pos >= pos {
                break;
            }
            
            if ch == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            
            byte_pos += ch.len_utf8();
        }
        
        (line, column)
    }

    fn update_position(&mut self, start: usize, end: usize) {
        // 确保 start 和 end 是字符边界
        let safe_start = self.source.char_indices()
            .find(|&(i, _)| i >= start)
            .map(|(i, _)| i)
            .unwrap_or(start.min(self.source.len()));
        
        let safe_end = self.source.char_indices()
            .find(|&(i, _)| i >= end)
            .map(|(i, _)| i)
            .unwrap_or(end.min(self.source.len()));
        
        // 使用字符迭代器而不是字节索引
        let slice = &self.source[safe_start..safe_end];
        for ch in slice.chars() {
            if ch == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } else {
                self.current_column += 1;
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_source_slice(&self, span: Span) -> &'a str {
        // 确保 start 和 end 是字符边界
        let safe_start = self.source.char_indices()
            .find(|&(i, _)| i >= span.start)
            .map(|(i, _)| i)
            .unwrap_or(span.start.min(self.source.len()));
        
        let safe_end = self.source.char_indices()
            .find(|&(i, _)| i >= span.end)
            .map(|(i, _)| i)
            .unwrap_or(span.end.min(self.source.len()));
        
        &self.source[safe_start..safe_end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "設 才 為 三";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Set);
        assert_eq!(tokens[1].token, Token::Var);
        assert_eq!(tokens[2].token, Token::Assign);
        assert_eq!(tokens[3].token, Token::Number("三".to_string()));
    }

    #[test]
    fn test_string_literal() {
        let source = "為「天工開物」曰";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::Assign);
        assert_eq!(tokens[1].token, Token::StringLiteral("天工開物".to_string()));
        assert_eq!(tokens[2].token, Token::Print);
    }

    #[test]
    fn test_identifier() {
        let source = "設 變量 為 值";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Set);
        assert_eq!(tokens[1].token, Token::Identifier("變量".to_string()));
        assert_eq!(tokens[2].token, Token::Assign);
        assert_eq!(tokens[3].token, Token::Identifier("值".to_string()));
    }

    #[test]
    fn test_operators() {
        let source = "一 加 二 等於 三";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token, Token::Number("一".to_string()));
        assert_eq!(tokens[1].token, Token::Add);
        assert_eq!(tokens[2].token, Token::Number("二".to_string()));
        assert_eq!(tokens[3].token, Token::Equal);
        assert_eq!(tokens[4].token, Token::Number("三".to_string()));
    }

    #[test]
    fn test_comment() {
        let source = "甲 為 三 注：省略設關鍵字的省略句";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        
        println!("Tokens for source: '{}'", source);
        for (i, token_info) in tokens.iter().enumerate() {
            println!("  Token {}: {:?}", i, token_info.token);
        }
        
        // 应该只有3个token：甲、為、三
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::Identifier("甲".to_string()));
        assert_eq!(tokens[1].token, Token::Assign);
        assert_eq!(tokens[2].token, Token::Number("三".to_string()));
    }
}