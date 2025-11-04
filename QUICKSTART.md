# HinCode 快速开始

> v0.1.0 MVP 使用指南
>
> 作者: ekent

## 🚀 快速上手

### 1. 构建项目

```bash
# 克隆仓库
git clone https://github.com/ekent/hincode
cd hincode

# 构建 release 版本
cargo build --release

# 二进制文件位于
./target/release/hincode
```

### 2. 设置 API Key

HinCode 需要 OpenAI API Key 才能工作:

```bash
export OPENAI_API_KEY="your-openai-api-key-here"
```

### 3. 使用 HinCode

在任何 Git 仓库中运行:

```bash
# 基础用法: 审查相对于 main 分支的变更
./target/release/hincode

# 指定 base 分支
./target/release/hincode --base develop

# 输出 Markdown 格式
./target/release/hincode --output markdown

# 输出 JSON 格式
./target/release/hincode --output json
```

## 📝 示例

### 示例 1: 基础审查

```bash
$ cd your-project
$ export OPENAI_API_KEY=sk-xxx
$ hincode

🔍 HinCode v0.1.0
AI-powered code review tool

📂 正在分析 Git 仓库...
发现 3 个变更文件

🔍 正在审查: src/main.rs (rust)
⏳ 正在调用 AI 审查... 完成!

🔍 AI Code Review Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. 🔴 Critical - 潜在空指针
   行号: 45
   代码中使用了 unwrap(),可能导致 panic

   💡 建议:
   使用 ? 操作符或 map_err 处理错误

   ────────────────────────

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 统计: 发现 2 个问题
```

### 示例 2: Markdown 输出 (用于 PR 评论)

```bash
$ hincode --output markdown > review.md
$ cat review.md

# 🔍 AI Code Review Report

## 🔴 Critical - 潜在 SQL 注入

**行号**: 45

代码中使用了字符串拼接构建 SQL 查询,可能导致 SQL 注入。

**建议**:
使用参数化查询...
```

## 🔧 配置

### 环境变量

| 变量 | 说明 | 必需 |
|------|------|------|
| `OPENAI_API_KEY` | OpenAI API 密钥 | ✅ 是 |

### 命令行选项

```
Options:
  -b, --base <BASE>      Base branch [default: main]
  -o, --output <OUTPUT>  Output format [default: terminal]
  -v, --verbose          Verbose output
  -h, --help             Print help
  -V, --version          Print version
```

## 💡 注意事项

### MVP 限制

v0.1.0 是最小可用版本,有以下限制:

- ✅ 只审查第一个变更文件 (完整版会审查所有)
- ✅ 暂不支持配置文件 (v0.2 会添加)
- ✅ 暂不支持交互式模式 (v0.4 会添加)
- ✅ AI 可能偶尔会产生误报 (持续优化中)

### 隐私

- HinCode 会将代码发送到 OpenAI API 进行审查
- 如果对隐私敏感,请等待本地模型支持 (v2.0 计划)

## 🐛 故障排查

### "无法打开 Git 仓库"

确保在 Git 仓库目录中运行 hincode:

```bash
git status  # 先检查是否是 Git 仓库
hincode
```

###  "请设置环境变量 OPENAI_API_KEY"

设置 API Key:

```bash
export OPENAI_API_KEY=sk-your-key
hincode
```

### "获取相对于 main 的变更失败"

指定正确的 base 分支:

```bash
git branch -a  # 查看所有分支
hincode --base origin/main  # 使用远程分支
```

## 🚀 下一步

- 提交 Issue 或 PR

## 📄 许可

MIT License - 详见 [LICENSE](LICENSE)

---

**作者**: ekent
**最后更新**: 2025-11-04
