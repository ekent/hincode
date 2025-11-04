//! HinCode - AI-powered code review tool
//!
//! 作者: ekent

use anyhow::{Context, Result};
use clap::Parser;
use hincode::{
    ai::{AIProvider, OpenAIProvider, ReviewRequest},
    cli::{Cli, OutputFormat, OutputGenerator},
    git::GitAnalyzer,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    println!("🔍 HinCode v{}", env!("CARGO_PKG_VERSION"));
    println!("AI-powered code review tool\n");

    // 1. 检查 API Key
    let api_key = env::var("OPENAI_API_KEY")
        .context("请设置环境变量 OPENAI_API_KEY\n提示: export OPENAI_API_KEY=your-key")?;

    // 2. 打开 Git 仓库
    println!("📂 正在分析 Git 仓库...");
    let analyzer = GitAnalyzer::open(".")
        .context("无法打开 Git 仓库。请确保在 Git 仓库目录中运行此命令。")?;

    // 3. 获取变更文件
    let files = analyzer
        .get_changed_files(&cli.base)
        .with_context(|| format!("获取相对于 {} 的变更失败", cli.base))?;

    if files.is_empty() {
        println!("✅ 相对于 {} 没有变更", cli.base);
        return Ok(());
    }

    println!("发现 {} 个变更文件\n", files.len());

    // 4. 审查文件 (MVP: 只审查第一个文件)
    if let Some(file) = files.first() {
        println!("🔍 正在审查: {} ({})", file.path, file.language);

        // 创建 AI provider
        let provider = OpenAIProvider::new(api_key);

        // 发送审查请求
        let request = ReviewRequest {
            file_path: file.path.clone(),
            content: file.get_content(),
            language: file.language.clone(),
        };

        print!("⏳ 正在调用 AI 审查...");
        let response = provider.review(request).await?;
        println!(" 完成!\n");

        // 5. 输出结果
        let format = OutputFormat::from_str(&cli.output);
        let output = OutputGenerator::generate(&response, format);
        println!("{}", output);
    }

    Ok(())
}
