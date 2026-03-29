# 天工語 (TianGong Lang)

## 簡介 | Introduction

**天工語者，基於文言文與繁體中文之高級編程語言也。**  
其融合古漢語之簡練與現代編程之邏輯，創造全新之編程體驗。  
語言設計遵循「天工開物，巧奪天工」之理念，力求簡潔優雅，功能完備。

**TianGong Lang is a high-level programming language based on Classical Chinese and Traditional Chinese.**  
It combines the conciseness of ancient Chinese with the logic of modern programming, creating a brand new programming experience.  
The language design follows the philosophy of "Heavenly Craftsmanship, Surpassing Nature", striving for simplicity, elegance, and completeness.

## 特色功能 | Features

### 文言文特色 | Classical Chinese Features
- **通假字支持**：`才`通`設`（變量聲明），`說`通`稅`（註釋）
- **倒裝句法**：`表達式 曰`（相當於：`曰 表達式`）
- **省略語法**：`變量 為 值`（省略`設`關鍵字）
- **古今異義**：`走`表循環，`知`表判斷，`曰`表輸出
- **中文數字**：`一、二、三、十、百、千、萬`

### 現代功能 | Modern Features
- **完整類型系統**：整數、浮點數、字符串、布爾值、數組、字典
- **控制結構**：條件判斷、循環、函數、異常處理
- **豐富標準庫**：數學、字符串、數組、字典、類型檢查等30+函數
- **交互環境**：增強型REPL，支持歷史記錄、幫助系統
- **錯誤處理**：詳細錯誤信息，包含源代碼上下文

## 安裝 | Installation

### 從源碼構建 | Build from Source
```bash
# 克隆倉庫 | Clone repository
git clone <repository-url>
cd tiangong-lang

# 構建項目 | Build project
cargo build --release

# 安裝到系統路徑 | Install to system path
cargo install --path .
```

### 使用預編譯二進制 | Use Precompiled Binary
從發佈頁面下載對應平台之二進制文件，添加到系統PATH中。  
Download the corresponding platform binary from the release page and add it to the system PATH.

## 快速開始 | Quick Start

### Hello World
```天工語
"你好，世界！" 曰
```

### 變量與運算 | Variables and Operations
```天工語
設 圓周率 為 三點一四一五九
設 半徑 為 五
設 面積 為 圓周率 乘 半徑 乘 半徑
面積 曰  # 輸出: 78.53975
```

### 條件語句 | Conditional Statements
```天工語
設 分數 為 八十五

若 分數 大於等於 九十 則
    輸出("優秀")
若否 分數 大於等於 八十 則
    輸出("良好")
若否 分數 大於等於 六十 則
    輸出("及格")
若然則
    輸出("不及格")
終
```

### 函數定義 | Function Definition
```天工語
謂 階乘(數)
    若 數 小於等於 一 則
        返 一
    終
    返 數 乘 執 階乘(數 減 一)
終

執 階乘(五) 曰  # 輸出: 120
```

## 使用方式 | Usage

### 運行程序 | Run Program
```bash
天工語 run examples/hello.tg
```

### 交互環境 | Interactive Environment (REPL)
```bash
天工語 repl
```

### 顯示分析結果 | Show Analysis Results
```bash
# 詞法分析 | Lexical Analysis
天工語 tokens examples/hello.tg

# 語法分析 | Syntax Analysis (AST)
天工語 parse examples/hello.tg
```

## 標準庫函數 | Standard Library Functions

### 輸入輸出 | Input/Output
- `輸出(...)` - 輸出多個值 | Output multiple values
- `輸入()` - 讀取用戶輸入 | Read user input

### 數學函數 | Mathematical Functions
- `絕對值(數)` - 返回絕對值 | Return absolute value
- `平方根(數)` - 返回平方根 | Return square root
- `四捨五入(數)` - 返回四捨五入值 | Return rounded value
- `最大值(...)` - 返回最大值 | Return maximum value
- `最小值(...)` - 返回最小值 | Return minimum value

### 字符串函數 | String Functions
- `長度(字符串)` - 返回長度 | Return length
- `轉字符串(值)` - 轉換為字符串 | Convert to string
- `大寫(字符串)` - 轉換為大寫 | Convert to uppercase
- `小寫(字符串)` - 轉換為小寫 | Convert to lowercase
- `修剪(字符串)` - 去除首尾空格 | Trim whitespace
- `分割(字符串,分隔符)` - 分割字符串 | Split string
- `連接(字符串1,字符串2)` - 連接字符串 | Concatenate strings

### 數組函數 | Array Functions
- `推入(數組,值)` - 添加元素到數組末尾 | Add element to array end
- `彈出(數組)` - 移除數組最後一個元素 | Remove last element from array
- `合併(數組1,數組2)` - 合併兩個數組 | Merge two arrays
- `切片(數組,起始,結束)` - 獲取數組切片 | Get array slice
- `範圍(起始,結束)` - 生成範圍數組 | Generate range array

### 類型檢查 | Type Checking
- `是數字(值)` - 檢查是否為數字 | Check if value is number
- `是字符串(值)` - 檢查是否為字符串 | Check if value is string
- `是數組(值)` - 檢查是否為數組 | Check if value is array
- `是字典(值)` - 檢查是否為字典 | Check if value is dictionary
- `是布爾(值)` - 檢查是否為布爾值 | Check if value is boolean
- `是空(值)` - 檢查是否為空值 | Check if value is null

## 項目結構 | Project Structure

```
天工語/
├── src/                    # 源代碼 | Source Code
│   ├── ast/              # 抽象語法樹 | Abstract Syntax Tree
│   ├── lexer/            # 詞法分析器 | Lexer
│   ├── parser/           # 語法分析器 | Parser
│   ├── runtime/          # 運行時系統 | Runtime System
│   │   ├── interpreter.rs # 解釋器 | Interpreter
│   │   ├── stdlib.rs     # 標準庫 | Standard Library
│   │   ├── value.rs      # 值類型 | Value Types
│   │   └── mod.rs        # 模塊聲明 | Module Declaration
│   ├── error.rs          # 錯誤處理 | Error Handling
│   ├── lib.rs            # 庫模塊聲明 | Library Module Declaration
│   └── main.rs           # 主程序 | Main Program
├── examples/              # 示例文件 | Example Files
│   ├── hello.tg          # Hello World
│   ├── math_operations.tg # 數學運算 | Math Operations
│   ├── string_operations.tg # 字符串操作 | String Operations
│   ├── array_operations.tg # 數組操作 | Array Operations
│   ├── control_flow.tg   # 控制流 | Control Flow
│   └── variables.tg      # 變量示例 | Variable Examples
├── Cargo.toml            # 項目配置 | Project Configuration
└── README.md             # 項目說明 | Project Documentation
```

## 開發狀態 | Development Status

### 已完成 | Completed
- ✅ 詞法分析器（支持文言文關鍵字、中文數字、字符串字面量）
- ✅ 語法分析器（支持變量聲明、控制流、函數等語法）
- ✅ 解釋器（完整運行時環境）
- ✅ 標準庫（30+個實用函數）
- ✅ 錯誤處理系統（9種錯誤類型，源代碼上下文）
- ✅ 交互環境（增強型REPL，歷史記錄、幫助系統）
- ✅ 測試套件（15個測試用例全部通過）

### 進行中 | In Progress
- 🔄 性能優化（字節碼編譯器、JIT編譯）
- 🔄 模塊系統（導入/導出功能）
- 🔄 擴展標準庫（文件操作、網絡功能）

### 計劃中 | Planned
- 📋 編譯器後端（生成機器碼）
- 📋 調試器（源代碼級調試）
- 📋 IDE插件（語法高亮、代碼補全）

## 技術棧 | Technology Stack

- **編程語言**：Rust（系統級編程，高性能）
- **詞法分析**：Logos（正則表達式詞法分析器生成器）
- **語法分析**：手寫遞歸下降解析器
- **序列化**：Serde（JSON序列化/反序列化）
- **時間處理**：Chrono（日期時間庫）

## 貢獻指南 | Contribution Guidelines

### 報告問題 | Reporting Issues
1. 查看現有問題是否已報告 | Check if issue already reported
2. 提供詳細重現步驟 | Provide detailed reproduction steps
3. 包含錯誤信息和代碼示例 | Include error messages and code examples

### 提交代碼 | Submitting Code
1. 閱讀架構文檔了解項目結構 | Read architecture documentation
2. 遵循現有代碼風格和約定 | Follow existing code style and conventions
3. 編寫測試用例 | Write test cases
4. 提交Pull Request | Submit Pull Request

### 文檔貢獻 | Documentation Contributions
1. 更新README和示例文件 | Update README and example files
2. 添加代碼註釋和文檔字符串 | Add code comments and docstrings
3. 翻譯文檔到其他語言 | Translate documentation to other languages

## 許可證 | License

本項目採用MIT許可證。詳見 [LICENSE](LICENSE) 文件。  
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 致謝 | Acknowledgments

- **靈感來源**：文言文編程語言、中國古代科技典籍《天工開物》
- **技術參考**：Rust編程語言、現代編譯器設計原理
- **貢獻者**：所有為本項目做出貢獻的開發者

## 聯繫方式 | Contact

- **GitHub Issues**：報告問題和建議 | Report issues and suggestions
- **文檔貢獻**：幫助完善文檔 | Help improve documentation
- **代碼貢獻**：提交改進和功能 | Submit improvements and features

---

**天工開物，巧奪天工**  
**Heavenly Craftsmanship, Surpassing Nature**

願天工語助您開啟編程新境界！  
May TianGong Lang help you open a new realm of programming!