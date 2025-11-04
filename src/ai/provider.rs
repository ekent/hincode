//! AI Provider 抽象

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// AI Provider trait
#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn review(&self, request: ReviewRequest) -> Result<ReviewResponse>;
}

/// 审查请求
#[derive(Debug, Clone)]
pub struct ReviewRequest {
    pub file_path: String,
    pub content: String,
    pub language: String,
}

/// 审查响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub issues: Vec<Issue>,
}

/// 问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: Severity,
    pub category: Category,
    pub title: String,
    pub description: String,
    pub line: u32,
    pub suggestion: String,
    pub confidence: f32,
}

/// 严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Critical, // 严重
    High,     // 高
    Medium,   // 中
    Low,      // 低
}

/// 问题类别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Category {
    Security,     // 安全
    Bug,          // Bug
    Performance,  // 性能
    Style,        // 风格
    BestPractice, // 最佳实践
}
