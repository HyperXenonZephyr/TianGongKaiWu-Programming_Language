use clap::{Arg, Command};
use std::fs;
use std::io::{self, Write};

mod lexer;
mod parser;
mod ast;
mod runtime;

use lexer::Lexer;
use parser::Parser;
use crate::runtime::interpreter::Interpreter;

fn main() {
    let matches = Command::new("天工語")
        .version("0.1.0")
        .author("天工語開發團隊")
        .about("天工開物編程語言 - 基於文言文與繁體中文的高級編程語言")
        .subcommand(
            Command::new("run")
                .about("運行天工語程序")
                .arg(Arg::new("file").help("要運行的文件").required(true)),
        )
        .subcommand(
            Command::new("repl")
                .about("啟動交互式環境"),
        )
        .subcommand(
            Command::new("parse")
                .about("解析文件並顯示AST")
                .arg(Arg::new("file").help("要解析的文件").required(true)),
        )
        .subcommand(
            Command::new("tokens")
                .about("顯示文件的詞法分析結果")
                .arg(Arg::new("file").help("要分析的文件").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();
            run_file(file_path);
        }
        Some(("repl", _)) => {
            run_repl();
        }
        Some(("parse", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();
            parse_file(file_path);
        }
        Some(("tokens", sub_matches)) => {
            let file_path = sub_matches.get_one::<String>("file").unwrap();
            show_tokens(file_path);
        }
        _ => {
            println!("請使用子命令：run, repl, parse, tokens");
            println!("使用 --help 查看詳細信息");
        }
    }
}

fn run_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => {
            if let Err(e) = run(&source) {
                eprintln!("運行錯誤: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("無法讀取文件 {}: {}", file_path, e);
            std::process::exit(1);
        }
    }
}

fn run(source: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    if tokens.is_empty() {
        return Ok(());
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program)?;
    
    Ok(())
}

fn run_repl() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                   天工語 REPL v0.1.0                    ║");
    println!("║        基於文言文與繁體中文的高級編程語言              ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("輸入 '退出' 或 'exit' 退出");
    println!("輸入 '幫助' 或 'help' 查看幫助");
    println!("輸入 '示例' 或 'examples' 查看示例");
    println!("輸入 '函數' 或 'functions' 查看可用函數");
    
    let mut interpreter = Interpreter::new();
    let mut history: Vec<String> = Vec::new();
    
    loop {
        print!("天工> ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim();
        
        match input {
            "退出" | "exit" => break,
            "幫助" | "help" => {
                show_help();
                continue;
            }
            "示例" | "examples" => {
                show_examples();
                continue;
            }
            "函數" | "functions" => {
                show_functions();
                continue;
            }
            "歷史" | "history" => {
                show_history(&history);
                continue;
            }
            "清屏" | "clear" => {
                clear_screen();
                continue;
            }
            "環境" | "env" => {
                show_environment(&interpreter);
                continue;
            }
            "" => continue,
            _ => {}
        }
        
        // 添加到历史
        history.push(input.to_string());
        if history.len() > 50 {
            history.remove(0);
        }
        
        // 执行代码
        match run_with_interpreter(input, &mut interpreter) {
            Ok(result) => {
                if !matches!(result, runtime::value::Value::Null) {
                    println!("結果: {}", result);
                }
            }
            Err(e) => eprintln!("錯誤: {}", e),
        }
    }
    
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                        再會！                           ║");
    println!("║                期待下次與您相會                         ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}

fn run_with_interpreter(source: &str, interpreter: &mut Interpreter) -> Result<runtime::value::Value, String> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    
    if tokens.is_empty() {
        return Ok(runtime::value::Value::Null);
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    interpreter.interpret(&program)
}

fn show_help() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                        幫助                              ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("命令:");
    println!("  退出/exit      - 退出REPL");
    println!("  幫助/help      - 顯示此幫助");
    println!("  示例/examples  - 查看示例代碼");
    println!("  函數/functions - 查看可用函數");
    println!("  歷史/history   - 查看歷史命令");
    println!("  清屏/clear     - 清空屏幕");
    println!("  環境/env       - 顯示當前環境變量");
    println!();
    println!("語法示例:");
    println!("  設 變量 為 值          # 變量聲明");
    println!("  變量 為 值             # 省略設關鍵字");
    println!("  表達式 曰              # 倒裝句輸出");
    println!("  若 條件 則 代碼 終     # 條件語句");
    println!("  走 次數 次 代碼 終     # 循環語句");
    println!("  謂 函數名(參數) 代碼 終 # 函數定義");
    println!("  執 函數名(參數)        # 函數調用");
}

fn show_examples() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                        示例                              ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("1. 基本運算:");
    println!("   一 加 二 曰                    # 輸出: 3");
    println!("   十 減 三 曰                    # 輸出: 7");
    println!("   四 乘 五 曰                    # 輸出: 20");
    println!("   二十 除 四 曰                  # 輸出: 5");
    println!();
    println!("2. 變量操作:");
    println!("   設 甲 為 十                    # 定義變量");
    println!("   甲 為 甲 加 一                 # 修改變量");
    println!("   甲 曰                          # 輸出變量值");
    println!();
    println!("3. 條件語句:");
    println!("   若 甲 大於 五 則");
    println!("       甲 曰");
    println!("   終");
    println!();
    println!("4. 函數定義:");
    println!("   謂 平方(數)");
    println!("       返 數 乘 數");
    println!("   終");
    println!("   執 平方(五) 曰                # 輸出: 25");
    println!();
    println!("5. 數組操作:");
    println!("   設 數組 為 [一, 二, 三]");
    println!("   數組[零] 曰                   # 輸出: 1");
    println!("   長度(數組) 曰                 # 輸出: 3");
}

fn show_functions() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                        可用函數                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!("輸入輸出:");
    println!("  輸出(...)      - 輸出多個值");
    println!("  輸入()         - 讀取用戶輸入");
    println!();
    println!("數學函數:");
    println!("  絕對值(數)     - 返回絕對值");
    println!("  平方根(數)     - 返回平方根");
    println!("  四捨五入(數)   - 返回四捨五入值");
    println!("  最大值(...)    - 返回最大值");
    println!("  最小值(...)    - 返回最小值");
    println!();
    println!("字符串函數:");
    println!("  長度(字符串)   - 返回長度");
    println!("  轉字符串(值)   - 轉換為字符串");
    println!("  大寫(字符串)   - 轉換為大寫");
    println!("  小寫(字符串)   - 轉換為小寫");
    println!("  修剪(字符串)   - 去除首尾空格");
    println!("  分割(字符串,分隔符) - 分割字符串");
    println!("  連接(字符串1,字符串2) - 連接字符串");
    println!();
    println!("數組函數:");
    println!("  推入(數組,值)  - 添加元素到數組末尾");
    println!("  彈出(數組)     - 移除數組最後一個元素");
    println!("  合併(數組1,數組2) - 合併兩個數組");
    println!("  切片(數組,起始,結束) - 獲取數組切片");
    println!();
    println!("類型檢查:");
    println!("  是數字(值)     - 檢查是否為數字");
    println!("  是字符串(值)   - 檢查是否為字符串");
    println!("  是數組(值)     - 檢查是否為數組");
    println!("  是字典(值)     - 檢查是否為字典");
    println!("  是布爾(值)     - 檢查是否為布爾值");
    println!("  是空(值)       - 檢查是否為空值");
}

fn show_history(history: &[String]) {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                        歷史命令                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    if history.is_empty() {
        println!("暫無歷史記錄");
    } else {
        for (i, cmd) in history.iter().enumerate() {
            println!("  {:3}: {}", i + 1, cmd);
        }
    }
}

fn show_environment(_interpreter: &Interpreter) {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                        環境變量                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    // 注意：这里需要访问interpreter的环境
    // 由于Environment结构没有公开的方法来获取所有变量，这里只显示提示
    println!("使用 變量名 曰 來查看變量值");
    println!("或查看源代碼中的環境實現");
}

fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    io::stdout().flush().unwrap();
}

fn parse_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => {
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.tokenize();
            
            println!("詞法分析結果 ({} 個token):", tokens.len());
            for (i, token_info) in tokens.iter().enumerate() {
                println!("  {}: {:?}", i, token_info.token);
            }
            
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(program) => {
                    println!("\n語法分析結果 (AST):");
                    println!("{:#?}", program);
                }
                Err(e) => {
                    eprintln!("解析錯誤: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("無法讀取文件 {}: {}", file_path, e);
        }
    }
}

fn show_tokens(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(source) => {
            let mut lexer = Lexer::new(&source);
            let tokens = lexer.tokenize();
            
            println!("文件: {}", file_path);
            println!("詞法分析結果 (共 {} 個token):\n", tokens.len());
            
            for token_info in tokens {
                let token_str = match &token_info.token {
                    lexer::Token::Number(n) => format!("數字({})", n),
                    lexer::Token::StringLiteral(s) => format!("字符串「{}」", s),
                    lexer::Token::Identifier(id) => format!("標識符({})", id),
                    _ => format!("{}", token_info.token),
                };
                
                println!("行 {} 列 {}: {}",
                    token_info.span.line,
                    token_info.span.column,
                    token_str
                );
            }
        }
        Err(e) => {
            eprintln!("無法讀取文件 {}: {}", file_path, e);
        }
    }
}

// 辅助函数，因为println!在Windows上可能有问题
fn println(text: &str) {
    println!("{}", text);
}