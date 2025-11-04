//! AI 模块 - AI 代码审查

mod provider;
mod openai;
mod prompt;

pub use provider::{AIProvider, Category, Issue, ReviewRequest, ReviewResponse, Severity};
pub use openai::OpenAIProvider;
pub use prompt::SYSTEM_PROMPT;
