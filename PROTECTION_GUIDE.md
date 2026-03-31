# 天工语项目 - Git保护措施指南

## 概述

本文档描述了为防止意外提交`target`目录和其他构建文件而实施的多层保护措施。经过历史清理后，仓库大小已从230MB减少到59KB，确保豆包AI可以正常解析。

## 保护措施层次

### 第一层：.gitignore配置
**位置**: `.gitignore`
**作用**: 阻止Git跟踪特定文件和目录

已配置的忽略规则包括：
- `target/` - Rust/Cargo构建目录
- `debug/` - 调试构建
- `Cargo.lock` - 依赖锁文件（库项目通常不提交）
- 各种构建产物、IDE文件、临时文件等

### 第二层：Git预提交钩子
**位置**: `.git/hooks/pre-commit*`
**作用**: 在提交前检查是否意外添加了被忽略的文件

#### 检查内容：
1. **target目录文件**：如果检测到`target/`目录下的文件，提交将被阻止
2. **Cargo.lock文件**：提交时会提示确认（库项目通常不提交）

#### 错误示例：
```
🔍 检查提交中是否包含target目录文件...
❌ 错误：检测到target目录文件被添加到提交中！
以下文件不应该被提交：
  target/debug/test.rlib

请从暂存区移除这些文件：
  git reset HEAD -- target/
  或
  git reset HEAD -- $targetFiles

然后添加正确的文件并重新提交。
```

### 第三层：Git强制保护
即使使用`git add -f`强制添加，预提交钩子仍会检查并阻止提交。

## 使用方法

### 正常开发流程
1. 编写代码
2. 运行测试：`cargo test`
3. 构建项目：`cargo build`
4. 检查状态：`git status`（应该看不到target目录）
5. 添加文件：`git add .` 或指定文件
6. 提交更改：`git commit -m "描述"`

### 如果意外添加了target文件
如果预提交钩子阻止了提交，按以下步骤修复：

```bash
# 1. 从暂存区移除target目录
git reset HEAD -- target/

# 2. 确认状态
git status

# 3. 重新添加正确的文件
git add src/ Cargo.toml README.md  # 只添加需要的文件

# 4. 重新提交
git commit -m "修复：移除误添加的target目录"
```

### 跳过钩子检查（不推荐）
仅在紧急情况下使用：
```bash
git commit -m "紧急提交" --no-verify
```

## 验证保护措施

### 测试.gitignore
```bash
# 创建测试文件
mkdir -p target/debug
echo "test" > target/debug/test.rlib

# 检查状态（应该看不到target目录）
git status

# 尝试添加（应该被阻止）
git add target/
```

### 测试预提交钩子
```bash
# 强制添加target文件
git add -f target/

# 尝试提交（应该被钩子阻止）
git commit -m "测试提交"
```

## 维护指南

### 更新.gitignore
如果需要添加新的忽略规则，编辑`.gitignore`文件并提交更改。

### 更新预提交钩子
编辑`.git/hooks/pre-commit.ps1`（PowerShell版本）或`.git/hooks/pre-commit`（Shell版本）。

### 分享钩子给其他开发者
Git钩子默认不随仓库共享。如果需要团队共享，可以考虑：
1. 将钩子脚本放在项目根目录（如`scripts/git-hooks/`）
2. 创建安装脚本
3. 在README中说明安装步骤

## 故障排除

### 钩子不执行
1. 检查文件权限：确保钩子文件可执行
2. 检查文件位置：应在`.git/hooks/`目录下
3. 检查Git配置：`git config core.hooksPath`

### .gitignore不生效
1. 检查规则语法
2. 清除缓存：`git rm -r --cached .` 然后重新添加
3. 确保文件未被之前提交过

### 需要临时禁用保护
```bash
# 临时重命名钩子文件
mv .git/hooks/pre-commit .git/hooks/pre-commit.disabled

# 完成操作后恢复
mv .git/hooks/pre-commit.disabled .git/hooks/pre-commit
```

## 历史记录

### 2026-03-31：实施保护措施
- 完善`.gitignore`配置，添加全面保护规则
- 添加Git预提交钩子，防止意外提交target目录
- 测试并验证所有保护措施正常工作
- 仓库大小：230MB → 59KB（减少99.97%）

### 之前的问题
- Git历史中包含1423个target目录文件（914MB）
- 导致GitHub仓库大小230MB，超过豆包AI的200MB限制
- 使用`git-filter-repo`清理历史，移除所有target文件

## 联系与支持

如果遇到问题或需要修改保护措施，请参考：
1. Git文档：https://git-scm.com/docs
2. .gitignore模式：https://git-scm.com/docs/gitignore
3. Git钩子：https://git-scm.com/docs/githooks

---
*最后更新：2026-03-31*  
*保护措施状态：✅ 正常运行*