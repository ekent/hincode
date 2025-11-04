# 🔍 HinCode

> AI-powered code review tool that understands your code
>
> **项目状态**: 📝 调研阶段 → 准备开发

---

## ⚠️ 开发中 / Work in Progress

这个项目目前正处于**调研和设计阶段**。如果你对此感兴趣,欢迎:

- ⭐ Star 这个仓库关注进展
- 💬 提出你的想法和建议

---

## 🎯 项目愿景

HinCode 旨在成为一个:

- ✅ **开源免费** - MIT 许可,永久免费
- ✅ **隐私优先** - 可本地运行,代码不上传
- ✅ **AI 驱动** - 深度理解代码语义
- ✅ **开箱即用** - 零配置,一键审查
- ✅ **CLI 优先** - 适配任意工作流

---

## 🚀 快速预览

### 未来的使用方式 (计划中)

```bash
# 安装 (计划支持)
cargo install hincode

# 审查当前分支相对 main 的改动
hincode

# 指定 base 分支和文件类型
hincode --base develop --files "*.rs"

# 专注安全问题
hincode --focus security

# 输出 Markdown 格式 (用于 PR 评论)
hincode --output markdown > review.md
```

### 期望的输出效果

```
🔍 AI Code Review Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

📁 src/auth.rs

  🔴 高风险 - 潜在 SQL 注入
  Line 45-48
  ┌─────────────────────────────────────
  │ let query = format!(
  │   "SELECT * FROM users WHERE id = {}",
  │   user_id
  │ );
  └─────────────────────────────────────

  💡 建议: 使用参数化查询
  let query = sqlx::query!(
    "SELECT * FROM users WHERE id = ?",
    user_id
  );

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📊 统计: 发现 5 个问题
   🔴 高风险: 1   🟡 中风险: 2   🔵 建议: 2
```

---

## 🗓️ 开发计划

### MVP (v0.1.0) - 目标: 2 周
- [ ] 项目初始化
- [ ] Git diff 提取 (git2-rs)
- [ ] OpenAI API 集成
- [ ] 基础 CLI 和输出

### 完善 (v0.2-v0.5) - 目标: 8 周
- [ ] 配置文件支持
- [ ] 多语言支持 (tree-sitter)
- [ ] Claude API 支持
- [ ] 交互式模式
- [ ] CI 集成

### 发布 (v1.0) - 目标: 12 周
- [ ] 文档完善
- [ ] 发布到 crates.io
- [ ] 公开营销
- [ ] 社区建设

---

## 🔧 技术栈 (计划)

- **语言**: Rust 1.75+
- **Git 操作**: git2-rs
- **代码解析**: tree-sitter (支持 50+ 语言)
- **AI 接口**: OpenAI API / Claude API
- **CLI 框架**: clap v4
- **输出美化**: colored, indicatif

---

## 🎯 差异化优势

vs 竞品 (CodeRabbit, GitHub Copilot, SonarQube):

| 特性 | HinCode | CodeRabbit | Copilot | SonarQube |
|------|---------|-----------|---------|-----------|
| 开源 | ✅ | ❌ | ❌ | 部分 |
| 本地运行 | ✅ | ❌ | ❌ | ✅ |
| AI 驱动 | ✅ | ✅ | ✅ | ❌ |
| 零配置 | ✅ | ✅ | ✅ | ❌ |
| CLI 优先 | ✅ | ❌ | ❌ | 部分 |
| 价格 | 免费 | $12/月 | $10/月 | $150+/年 |

---

## 🤝 参与贡献

### 现阶段你可以:

1. **提供反馈** - 对项目想法提出建议
2. **分享经验** - 你在代码审查中遇到的痛点
3. **Star 仓库** - 关注项目进展
4. **传播项目** - 分享给可能感兴趣的朋友

### 未来欢迎:

- 🐛 提交 Bug 报告
- 💡 提出功能建议
- 🔧 提交 Pull Request
- 📝 改进文档
- 🌍 翻译支持

---

## 📄 许可协议

[MIT License](LICENSE)

Copyright (c) 2025 ekent

**这意味着:**
- ✅ 可以免费使用
- ✅ 可以商业使用
- ✅ 可以修改和分发
- ✅ 只需保留版权声明

---

## 💬 联系方式

- **GitHub Issues**: 提问和讨论 (推荐)
- **作者**: ekent
- ekentqin@hincode.com

---

## 🙏 致谢

感谢所有关注和支持这个项目的人!

如果你认为这个项目有价值,欢迎:
- ⭐ Star 这个仓库
- 📢 分享给朋友
- 💬 提供反馈

---

## 📊 项目统计

- **调研文档**: 7 篇
- **总规划字数**: 30,000+ 字
- **预计 MVP 时间**: 2 周
- **预计 v1.0 时间**: 3 个月

---

**状态**: 🏗️ 调研完成,准备开发

**下一步**: 初始化 Rust 项目,开始 MVP 开发

---

*"好的开始是成功的一半。充分的规划,才能高效执行。"*

**让我们开始构建吧! 🚀**

---

**最后更新**: 2025-11-04
