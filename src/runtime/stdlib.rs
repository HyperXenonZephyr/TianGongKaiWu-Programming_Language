use crate::runtime::value::{Environment, Value};

/// 天工语标准库
pub struct StandardLibrary;

impl StandardLibrary {
    /// 初始化标准库，将所有函数添加到环境中
    pub fn init(env: &mut Environment) {
        Self::add_io_functions(env);
        Self::add_math_functions(env);
        Self::add_string_functions(env);
        Self::add_array_functions(env);
        Self::add_dict_functions(env);
        Self::add_type_functions(env);
        Self::add_utility_functions(env);
        Self::add_conversion_functions(env);
        Self::add_random_functions(env);
    }

    /// 添加输入输出函数
    fn add_io_functions(env: &mut Environment) {
        env.define(
            "輸出".to_string(),
            Value::NativeFunction(|args| {
                for arg in args {
                    print!("{}", arg);
                }
                println!();
                Ok(Value::Null)
            }),
        );

        env.define(
            "輸入".to_string(),
            Value::NativeFunction(|_args| {
                use std::io::{self, Write};
                io::stdout().flush().unwrap();
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                
                // 移除换行符
                let input = input.trim().to_string();
                Ok(Value::String(input))
            }),
        );
    }

    /// 添加数学函数
    fn add_math_functions(env: &mut Environment) {
        env.define(
            "絕對值".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(n.abs())),
                    Value::Integer(i) => Ok(Value::Integer(i.abs())),
                    _ => Err("參數必須是數字".to_string()),
                }
            }),
        );

        env.define(
            "平方根".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Number(n) => {
                        if *n < 0.0 {
                            Err("不能對負數取平方根".to_string())
                        } else {
                            Ok(Value::Number(n.sqrt()))
                        }
                    }
                    Value::Integer(i) => {
                        if *i < 0 {
                            Err("不能對負數取平方根".to_string())
                        } else {
                            Ok(Value::Number((*i as f64).sqrt()))
                        }
                    }
                    _ => Err("參數必須是數字".to_string()),
                }
            }),
        );

        env.define(
            "四捨五入".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Integer(n.round() as i64)),
                    Value::Integer(i) => Ok(Value::Integer(*i)),
                    _ => Err("參數必須是數字".to_string()),
                }
            }),
        );

        env.define(
            "最大值".to_string(),
            Value::NativeFunction(|args| {
                if args.is_empty() {
                    return Err("至少需要一個參數".to_string());
                }
                
                let mut max_value = None;
                
                for arg in args {
                    match arg {
                        Value::Number(n) => {
                            if let Some(Value::Number(current_max)) = max_value {
                                if n > current_max {
                                    max_value = Some(Value::Number(n));
                                }
                            } else {
                                max_value = Some(Value::Number(n));
                            }
                        }
                        Value::Integer(i) => {
                            if let Some(Value::Integer(current_max)) = max_value {
                                if i > current_max {
                                    max_value = Some(Value::Integer(i));
                                }
                            } else {
                                max_value = Some(Value::Integer(i));
                            }
                        }
                        _ => return Err("參數必須是數字".to_string()),
                    }
                }
                
                max_value.ok_or("無法計算最大值".to_string())
            }),
        );

        env.define(
            "最小值".to_string(),
            Value::NativeFunction(|args| {
                if args.is_empty() {
                    return Err("至少需要一個參數".to_string());
                }
                
                let mut min_value = None;
                
                for arg in args {
                    match arg {
                        Value::Number(n) => {
                            if let Some(Value::Number(current_min)) = min_value {
                                if n < current_min {
                                    min_value = Some(Value::Number(n));
                                }
                            } else {
                                min_value = Some(Value::Number(n));
                            }
                        }
                        Value::Integer(i) => {
                            if let Some(Value::Integer(current_min)) = min_value {
                                if i < current_min {
                                    min_value = Some(Value::Integer(i));
                                }
                            } else {
                                min_value = Some(Value::Integer(i));
                            }
                        }
                        _ => return Err("參數必須是數字".to_string()),
                    }
                }
                
                min_value.ok_or("無法計算最小值".to_string())
            }),
        );
    }

    /// 添加字符串函数
    fn add_string_functions(env: &mut Environment) {
        env.define(
            "長度".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                    Value::Array(arr) => Ok(Value::Integer(arr.len() as i64)),
                    Value::Dict(dict) => Ok(Value::Integer(dict.len() as i64)),
                    _ => Err("參數類型錯誤".to_string()),
                }
            }),
        );

        env.define(
            "轉字符串".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::String(args[0].to_string()))
            }),
        );

        env.define(
            "大寫".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_uppercase())),
                    _ => Err("參數必須是字符串".to_string()),
                }
            }),
        );

        env.define(
            "小寫".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_lowercase())),
                    _ => Err("參數必須是字符串".to_string()),
                }
            }),
        );

        env.define(
            "修剪".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.trim().to_string())),
                    _ => Err("參數必須是字符串".to_string()),
                }
            }),
        );

        env.define(
            "分割".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err("期望兩個參數".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::String(s), Value::String(delimiter)) => {
                        let parts: Vec<Value> = s
                            .split(delimiter)
                            .map(|part| Value::String(part.to_string()))
                            .collect();
                        Ok(Value::Array(parts))
                    }
                    _ => Err("參數必須是字符串".to_string()),
                }
            }),
        );

        env.define(
            "連接".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err("期望兩個參數".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::String(s1), Value::String(s2)) => {
                        Ok(Value::String(format!("{}{}", s1, s2)))
                    }
                    _ => Err("參數必須是字符串".to_string()),
                }
            }),
        );
    }

    /// 添加数组函数
    fn add_array_functions(env: &mut Environment) {
        env.define(
            "推入".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err("期望兩個參數".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        new_arr.push(args[1].clone());
                        Ok(Value::Array(new_arr))
                    }
                    _ => Err("第一個參數必須是數組".to_string()),
                }
            }),
        );

        env.define(
            "彈出".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        if arr.is_empty() {
                            Err("數組為空".to_string())
                        } else {
                            let mut new_arr = arr.clone();
                            new_arr.pop();
                            Ok(Value::Array(new_arr))
                        }
                    }
                    _ => Err("參數必須是數組".to_string()),
                }
            }),
        );

        env.define(
            "合併".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err("期望兩個參數".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr1), Value::Array(arr2)) => {
                        let mut new_arr = arr1.clone();
                        new_arr.extend(arr2.clone());
                        Ok(Value::Array(new_arr))
                    }
                    _ => Err("參數必須是數組".to_string()),
                }
            }),
        );

        env.define(
            "切片".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 3 {
                    return Err("期望三個參數".to_string());
                }
                match (&args[0], &args[1], &args[2]) {
                    (Value::Array(arr), Value::Integer(start), Value::Integer(end)) => {
                        if *start < 0 || *end > arr.len() as i64 || *start > *end {
                            return Err("索引範圍無效".to_string());
                        }
                        let slice = arr[*start as usize..*end as usize].to_vec();
                        Ok(Value::Array(slice))
                    }
                    _ => Err("參數類型錯誤".to_string()),
                }
            }),
        );
    }

    /// 添加字典函数
    fn add_dict_functions(env: &mut Environment) {
        env.define(
            "鍵".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Dict(dict) => {
                        let keys: Vec<Value> = dict
                            .keys()
                            .map(|k| Value::String(k.clone()))
                            .collect();
                        Ok(Value::Array(keys))
                    }
                    _ => Err("參數必須是字典".to_string()),
                }
            }),
        );

        env.define(
            "值".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Dict(dict) => {
                        let values: Vec<Value> = dict.values().cloned().collect();
                        Ok(Value::Array(values))
                    }
                    _ => Err("參數必須是字典".to_string()),
                }
            }),
        );

        env.define(
            "包含".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err("期望兩個參數".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::Dict(dict), Value::String(key)) => {
                        Ok(Value::Boolean(dict.contains_key(key)))
                    }
                    _ => Err("參數類型錯誤".to_string()),
                }
            }),
        );
    }

    /// 添加类型检查函数
    fn add_type_functions(env: &mut Environment) {
        env.define(
            "是數字".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::Boolean(matches!(args[0], Value::Number(_) | Value::Integer(_))))
            }),
        );

        env.define(
            "是字符串".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::Boolean(matches!(args[0], Value::String(_))))
            }),
        );

        env.define(
            "是數組".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::Boolean(matches!(args[0], Value::Array(_))))
            }),
        );

        env.define(
            "是字典".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::Boolean(matches!(args[0], Value::Dict(_))))
            }),
        );

        env.define(
            "是布爾".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::Boolean(matches!(args[0], Value::Boolean(_))))
            }),
        );

        env.define(
            "是空".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::Boolean(matches!(args[0], Value::Null)))
            }),
        );
    }

    /// 添加类型转换函数
    fn add_conversion_functions(env: &mut Environment) {
        env.define(
            "轉整數".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Integer(*n as i64)),
                    Value::Integer(i) => Ok(Value::Integer(*i)),
                    Value::String(s) => s.parse::<i64>()
                        .map(Value::Integer)
                        .map_err(|_| format!("無法將「{}」轉換為整數", s)),
                    Value::Boolean(b) => Ok(Value::Integer(if *b { 1 } else { 0 })),
                    _ => Err("無法轉換為整數".to_string()),
                }
            }),
        );

        env.define(
            "轉數字".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                match &args[0] {
                    Value::Number(n) => Ok(Value::Number(*n)),
                    Value::Integer(i) => Ok(Value::Number(*i as f64)),
                    Value::String(s) => s.parse::<f64>()
                        .map(Value::Number)
                        .map_err(|_| format!("無法將「{}」轉換為數字", s)),
                    Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
                    _ => Err("無法轉換為數字".to_string()),
                }
            }),
        );

        env.define(
            "類型".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 1 {
                    return Err("期望一個參數".to_string());
                }
                Ok(Value::String(args[0].type_name().to_string()))
            }),
        );
    }

    /// 添加随机数函数
    fn add_random_functions(env: &mut Environment) {
        env.define(
            "隨機".to_string(),
            Value::NativeFunction(|args| {
                use std::time::{SystemTime, UNIX_EPOCH};
                let seed = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .subsec_nanos();

                if args.is_empty() {
                    // 返回 0 到 1 之间的随机浮点数
                    let val = (seed as f64 % 10000.0) / 10000.0;
                    Ok(Value::Number(val))
                } else if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (Value::Integer(min), Value::Integer(max)) => {
                            if min >= max {
                                return Err("最小值必須小於最大值".to_string());
                            }
                            let range = (max - min) as u32;
                            let val = *min + (seed % range) as i64;
                            Ok(Value::Integer(val))
                        }
                        _ => Err("參數必須是整數".to_string()),
                    }
                } else {
                    Err("期望零個或兩個參數".to_string())
                }
            }),
        );
    }

    /// 添加工具函数
    fn add_utility_functions(env: &mut Environment) {
        env.define(
            "範圍".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err("期望兩個參數".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::Integer(start), Value::Integer(end)) => {
                        let range: Vec<Value> = (*start..*end)
                            .map(|i| Value::Integer(i))
                            .collect();
                        Ok(Value::Array(range))
                    }
                    _ => Err("參數必須是整數".to_string()),
                }
            }),
        );

        env.define(
            "重複".to_string(),
            Value::NativeFunction(|args| {
                if args.len() != 2 {
                    return Err("期望兩個參數".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::String(s), Value::Integer(count)) => {
                        if *count < 0 {
                            return Err("重複次數不能為負".to_string());
                        }
                        let repeated = s.repeat(*count as usize);
                        Ok(Value::String(repeated))
                    }
                    _ => Err("參數類型錯誤".to_string()),
                }
            }),
        );

        env.define(
            "現在".to_string(),
            Value::NativeFunction(|_args| {
                use chrono::Local;
                let now = Local::now();
                Ok(Value::String(now.format("%Y-%m-%d %H:%M:%S").to_string()))
            }),
        );
    }
}