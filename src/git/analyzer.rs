//! Git diff 分析器

use anyhow::{Context, Result};
use git2::{Diff, DiffOptions, Repository};
use std::path::Path;

/// Git 分析器
pub struct GitAnalyzer {
    repo: Repository,
}

impl GitAnalyzer {
    /// 打开 Git 仓库
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path)
            .context("无法打开 Git 仓库。请确保当前目录是一个 Git 仓库。")?;
        Ok(Self { repo })
    }

    /// 获取相对于 base 分支的变更文件
    pub fn get_changed_files(&self, base: &str) -> Result<Vec<FileDiff>> {
        // 1. 获取 base 分支的 tree
        let base_tree = self.get_branch_tree(base)?;

        // 2. 获取当前 HEAD 的 tree
        let head_tree = self.get_head_tree()?;

        // 3. 计算 diff
        let mut diff_opts = DiffOptions::new();
        diff_opts.context_lines(5); // 上下文行数

        let diff = self.repo.diff_tree_to_tree(
            Some(&base_tree),
            Some(&head_tree),
            Some(&mut diff_opts),
        )?;

        // 4. 提取文件变更
        self.extract_file_diffs(&diff)
    }

    /// 获取分支的 tree
    fn get_branch_tree(&self, branch_name: &str) -> Result<git2::Tree<'_>> {
        let obj = self
            .repo
            .revparse_single(branch_name)
            .with_context(|| format!("无法找到分支: {}", branch_name))?;
        let commit = obj.peel_to_commit()?;
        Ok(commit.tree()?)
    }

    /// 获取 HEAD 的 tree
    fn get_head_tree(&self) -> Result<git2::Tree<'_>> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        Ok(commit.tree()?)
    }

    /// 提取文件变更
    fn extract_file_diffs(&self, diff: &Diff) -> Result<Vec<FileDiff>> {
        let mut files = Vec::new();

        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    // 简化版本:只收集文件路径
                    files.push(FileDiff {
                        path: path.to_string_lossy().to_string(),
                        language: detect_language(path),
                        hunks: vec![],
                    });
                }
                true
            },
            None,
            None,
            None,
        )?;

        Ok(files)
    }
}

/// 文件变更
#[derive(Debug, Clone)]
pub struct FileDiff {
    pub path: String,
    pub language: String,
    pub hunks: Vec<DiffHunk>,
}

impl FileDiff {
    /// 获取文件内容(用于AI审查)
    pub fn get_content(&self) -> String {
        // MVP: 返回文件路径,后续版本读取实际内容
        format!("文件: {}", self.path)
    }
}

/// 变更块
#[derive(Debug, Clone)]
pub struct DiffHunk {
    pub old_start: u32,
    pub new_start: u32,
    pub lines: Vec<DiffLine>,
}

/// 变更行
#[derive(Debug, Clone)]
pub struct DiffLine {
    pub line_type: LineType,
    pub content: String,
    pub line_number: u32,
}

/// 行类型
#[derive(Debug, Clone, PartialEq)]
pub enum LineType {
    Added,
    Removed,
    Context,
}

/// 检测语言
fn detect_language(path: &Path) -> String {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => "rust".to_string(),
        Some("py") => "python".to_string(),
        Some("js") => "javascript".to_string(),
        Some("ts") => "typescript".to_string(),
        Some("go") => "go".to_string(),
        Some("java") => "java".to_string(),
        Some("cpp") | Some("cc") | Some("cxx") => "cpp".to_string(),
        Some("c") | Some("h") => "c".to_string(),
        _ => "unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        use std::path::PathBuf;

        assert_eq!(detect_language(&PathBuf::from("main.rs")), "rust");
        assert_eq!(detect_language(&PathBuf::from("app.py")), "python");
        assert_eq!(detect_language(&PathBuf::from("index.js")), "javascript");
    }
}
