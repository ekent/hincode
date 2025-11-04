//! 输出格式化

use crate::ai::{ReviewResponse, Severity};
use colored::Colorize;

pub enum OutputFormat {
    Terminal,
    Markdown,
    Json,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Self::Markdown,
            "json" => Self::Json,
            _ => Self::Terminal,
        }
    }
}

pub struct OutputGenerator;

impl OutputGenerator {
    pub fn generate(response: &ReviewResponse, format: OutputFormat) -> String {
        match format {
            OutputFormat::Terminal => Self::terminal_output(response),
            OutputFormat::Markdown => Self::markdown_output(response),
            OutputFormat::Json => {
                serde_json::to_string_pretty(response).unwrap_or_else(|_| "{}".to_string())
            }
        }
    }

    fn terminal_output(response: &ReviewResponse) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n{}\n", "🔍 AI Code Review Report".bold()));
        output.push_str(&"━".repeat(60));
        output.push_str("\n\n");

        if response.issues.is_empty() {
            output.push_str(&"✅ 未发现问题。代码看起来不错!\n".green().to_string());
            return output;
        }

        for (i, issue) in response.issues.iter().enumerate() {
            let icon = match issue.severity {
                Severity::Critical => "🔴",
                Severity::High => "🟠",
                Severity::Medium => "🟡",
                Severity::Low => "🔵",
            };

            output.push_str(&format!(
                "{}. {} {:?} - {}\n",
                i + 1,
                icon,
                issue.severity,
                issue.title.bold()
            ));

            output.push_str(&format!("   行号: {}\n", issue.line));
            output.push_str(&format!("   {}\n", issue.description.dimmed()));
            output.push_str(&format!("\n   💡 建议:\n   {}\n", issue.suggestion.italic()));
            output.push_str("\n   ────────────────────────\n\n");
        }

        // 统计
        output.push_str(&"━".repeat(60));
        output.push_str(&format!(
            "\n📊 统计: 发现 {} 个问题\n",
            response.issues.len()
        ));

        output
    }

    fn markdown_output(response: &ReviewResponse) -> String {
        let mut output = String::new();

        output.push_str("# 🔍 AI Code Review Report\n\n");

        if response.issues.is_empty() {
            output.push_str("✅ 未发现问题。代码看起来不错!\n");
            return output;
        }

        for issue in &response.issues {
            let severity_emoji = match issue.severity {
                Severity::Critical => "🔴",
                Severity::High => "🟠",
                Severity::Medium => "🟡",
                Severity::Low => "🔵",
            };

            output.push_str(&format!(
                "## {} {:?} - {}\n\n",
                severity_emoji, issue.severity, issue.title
            ));
            output.push_str(&format!("**行号**: {}\n\n", issue.line));
            output.push_str(&format!("{}\n\n", issue.description));
            output.push_str(&format!("**建议**:\n{}\n\n", issue.suggestion));
            output.push_str("---\n\n");
        }

        output.push_str(&format!("\n📊 **统计**: 发现 {} 个问题\n", response.issues.len()));

        output
    }
}
