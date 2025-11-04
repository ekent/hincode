//! 集成测试

use hincode::git::GitAnalyzer;

#[test]
fn test_open_current_repo() {
    // 测试打开当前仓库
    let result = GitAnalyzer::open(".");
    assert!(result.is_ok(), "应该能成功打开当前 Git 仓库");
}

#[test]
fn test_open_invalid_repo() {
    // 测试打开无效仓库
    let result = GitAnalyzer::open("/tmp/nonexistent");
    assert!(result.is_err(), "应该无法打开不存在的仓库");
}
