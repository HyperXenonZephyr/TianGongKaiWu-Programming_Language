use crate::lexer::Span;

/// 天工语错误类型
#[derive(Debug, Clone)]
pub enum Error {
    /// 词法分析错误
    Lexical(String, Span),
    
    /// 语法分析错误
    Syntax(String, Span),
    
    /// 运行时错误
    Runtime(String, Span),
    
    /// 类型错误
    TypeError(String, Span),
    
    /// 名称错误（未定义的变量或函数）
    NameError(String, Span),
    
    /// 参数错误
    ArgumentError(String, Span),
    
    /// 索引错误
    IndexError(String, Span),
    
    /// 除零错误
    DivisionByZero(Span),
    
    /// 文件错误
    FileError(String),
}

impl Error {
    /// 创建词法分析错误
    pub fn lexical(message: impl Into<String>, span: Span) -> Self {
        Error::Lexical(message.into(), span)
    }
    
    /// 创建语法分析错误
    pub fn syntax(message: impl Into<String>, span: Span) -> Self {
        Error::Syntax(message.into(), span)
    }
    
    /// 创建运行时错误
    pub fn runtime(message: impl Into<String>, span: Span) -> Self {
        Error::Runtime(message.into(), span)
    }
    
    /// 创建类型错误
    pub fn type_error(message: impl Into<String>, span: Span) -> Self {
        Error::TypeError(message.into(), span)
    }
    
    /// 创建名称错误
    pub fn name_error(message: impl Into<String>, span: Span) -> Self {
        Error::NameError(message.into(), span)
    }
    
    /// 创建参数错误
    pub fn argument_error(message: impl Into<String>, span: Span) -> Self {
        Error::ArgumentError(message.into(), span)
    }
    
    /// 创建索引错误
    pub fn index_error(message: impl Into<String>, span: Span) -> Self {
        Error::IndexError(message.into(), span)
    }
    
    /// 创建除零错误
    pub fn division_by_zero(span: Span) -> Self {
        Error::DivisionByZero(span)
    }
    
    /// 创建文件错误
    pub fn file_error(message: impl Into<String>) -> Self {
        Error::FileError(message.into())
    }
    
    /// 获取错误消息
    pub fn message(&self) -> String {
        match self {
            Error::Lexical(msg, _) => format!("詞法錯誤：{}", msg),
            Error::Syntax(msg, _) => format!("語法錯誤：{}", msg),
            Error::Runtime(msg, _) => format!("運行時錯誤：{}", msg),
            Error::TypeError(msg, _) => format!("類型錯誤：{}", msg),
            Error::NameError(msg, _) => format!("名稱錯誤：{}", msg),
            Error::ArgumentError(msg, _) => format!("參數錯誤：{}", msg),
            Error::IndexError(msg, _) => format!("索引錯誤：{}", msg),
            Error::DivisionByZero(_) => "除零錯誤".to_string(),
            Error::FileError(msg) => format!("文件錯誤：{}", msg),
        }
    }
    
    /// 获取带源代码上下文的错误消息
    pub fn message_with_context(&self, source: &str) -> String {
        let base_msg = self.message();
        if let Some(span) = self.span() {
            if span.start < source.len() && span.end <= source.len() {
                let line_start = source[..span.start].rfind('\n').map(|i| i + 1).unwrap_or(0);
                let line_end = source[span.end..].find('\n').map(|i| span.end + i).unwrap_or(source.len());
                let line = &source[line_start..line_end];
                let column = span.start - line_start + 1;
                let line_num = source[..span.start].chars().filter(|&c| c == '\n').count() + 1;
                
                let pointer = " ".repeat(column - 1) + "^";
                return format!("{}\n在第 {} 行，第 {} 列:\n{}\n{}", base_msg, line_num, column, line, pointer);
            }
        }
        base_msg
    }
    
    /// 获取带源代码片段上下文的错误消息（使用Lexer的get_source_slice）
    pub fn message_with_source_slice(&self, lexer: &crate::lexer::Lexer) -> String {
        let base_msg = self.message();
        if let Some(span) = self.span() {
            let source_slice = lexer.get_source_slice(span);
            if !source_slice.is_empty() {
                return format!("{}: {}", base_msg, source_slice);
            }
        }
        base_msg
    }
    
    /// 获取错误位置
    pub fn span(&self) -> Option<Span> {
        match self {
            Error::Lexical(_, span) => Some(*span),
            Error::Syntax(_, span) => Some(*span),
            Error::Runtime(_, span) => Some(*span),
            Error::TypeError(_, span) => Some(*span),
            Error::NameError(_, span) => Some(*span),
            Error::ArgumentError(_, span) => Some(*span),
            Error::IndexError(_, span) => Some(*span),
            Error::DivisionByZero(span) => Some(*span),
            Error::FileError(_) => None,
        }
    }
    
    /// 格式化错误信息，包含源代码上下文
    pub fn format_with_context(&self, source: &str) -> String {
        let message = self.message();
        
        if let Some(span) = self.span() {
            let line_start = span.line;
            let col_start = span.column;
            
            // 获取错误行
            let lines: Vec<&str> = source.lines().collect();
            if line_start <= lines.len() {
                let line_content = lines[line_start - 1];
                
                // 构建错误指示器
                let indicator = " ".repeat(col_start - 1) + "^";
                
                format!(
                    "{}\n在第 {} 行，第 {} 列：\n{}\n{}",
                    message, line_start, col_start, line_content, indicator
                )
            } else {
                format!("{}\n位置：第 {} 行，第 {} 列", message, line_start, col_start)
            }
        } else {
            message
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for Error {}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;