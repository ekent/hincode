//! OpenAI API 实现

use super::provider::{AIProvider, ReviewRequest, ReviewResponse};
use super::prompt::SYSTEM_PROMPT;
use anyhow::{Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

/// OpenAI Provider
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: "gpt-4-turbo-preview".to_string(),
        }
    }
}

#[async_trait]
impl AIProvider for OpenAIProvider {
    async fn review(&self, request: ReviewRequest) -> Result<ReviewResponse> {
        let prompt = format!(
            r#"请审查以下代码:

文件: {}
语言: {}

代码内容:
```
{}
```

请返回 JSON 数组格式的问题列表。
"#,
            request.file_path, request.language, request.content
        );

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": self.model,
                "messages": [
                    {"role": "system", "content": SYSTEM_PROMPT},
                    {"role": "user", "content": prompt}
                ],
                "temperature": 0.3,
            }))
            .send()
            .await
            .context("OpenAI API 请求失败")?;

        let response_json: OpenAIResponse = response
            .json()
            .await
            .context("解析 OpenAI 响应失败")?;

        // 解析 AI 返回的 JSON
        let content = &response_json.choices[0].message.content;
        let issues = serde_json::from_str(content).unwrap_or_else(|_| vec![]);

        Ok(ReviewResponse { issues })
    }
}

#[derive(Debug, Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Debug, Deserialize)]
struct Message {
    content: String,
}
