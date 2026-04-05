use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Integer(i64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function(Rc<Function>),
    NativeFunction(NativeFunction),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Dict(a), Value::Dict(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            // NativeFunction 不比较函数指针，只比较类型
            (Value::NativeFunction(_), Value::NativeFunction(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: crate::ast::Program,
    pub closure: Environment,
}

pub type NativeFunction = fn(Vec<Value>) -> Result<Value, String>;

#[derive(Debug, Clone)]
pub struct Environment {
    pub variables: HashMap<String, Value>,
    pub parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.variables.get(name).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get(name))
        })
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    /// 列出所有用户定义的变量（排除原生函数）
    pub fn list_variables(&self) -> Vec<(String, String)> {
        self.variables
            .iter()
            .filter(|(_, v)| !matches!(v, Value::NativeFunction(_)))
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect()
    }
}

impl Value {
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Number(_) => "數值",
            Value::Integer(_) => "整數",
            Value::String(_) => "字符串",
            Value::Boolean(_) => "布爾",
            Value::Null => "無",
            Value::Array(_) => "數組",
            Value::Dict(_) => "字典",
            Value::Function(_) => "函數",
            Value::NativeFunction(_) => "原生函數",
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Boolean(b) => *b,
            Value::Null => false,
            Value::Number(n) => *n != 0.0,
            Value::Integer(i) => *i != 0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Dict(d) => !d.is_empty(),
            _ => true,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => format!("{}", n),
            Value::Integer(i) => format!("{}", i),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "真".to_string() } else { "假".to_string() },
            Value::Null => "無".to_string(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Dict(dict) => {
                let items: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Function(func) => format!("函數<{}>", func.name),
            Value::NativeFunction(_) => "原生函數".to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.params == other.params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_truthy() {
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
        assert!(!Value::Null.is_truthy());
        assert!(Value::Number(1.0).is_truthy());
        assert!(!Value::Number(0.0).is_truthy());
        assert!(Value::Integer(1).is_truthy());
        assert!(!Value::Integer(0).is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String("".to_string()).is_truthy());
        assert!(Value::Array(vec![Value::Number(1.0)]).is_truthy());
        assert!(!Value::Array(vec![]).is_truthy());
    }

    #[test]
    fn test_value_to_string() {
        assert_eq!(Value::Number(3.14).to_string(), "3.14");
        assert_eq!(Value::Integer(42).to_string(), "42");
        assert_eq!(Value::String("測試".to_string()).to_string(), "測試");
        assert_eq!(Value::Boolean(true).to_string(), "真");
        assert_eq!(Value::Boolean(false).to_string(), "假");
        assert_eq!(Value::Null.to_string(), "無");
        
        let array = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
        assert_eq!(array.to_string(), "[1, 2]");
        
        let mut dict = HashMap::new();
        dict.insert("鍵".to_string(), Value::String("值".to_string()));
        let dict_value = Value::Dict(dict);
        assert_eq!(dict_value.to_string(), "{鍵: 值}");
    }

    #[test]
    fn test_environment() {
        let mut env = Environment::new();
        env.define("變量".to_string(), Value::Number(42.0));
        
        assert_eq!(env.get("變量"), Some(Value::Number(42.0)));
        assert_eq!(env.get("不存在"), None);
        
        env.set("變量".to_string(), Value::Number(100.0));
        assert_eq!(env.get("變量"), Some(Value::Number(100.0)));
    }
}