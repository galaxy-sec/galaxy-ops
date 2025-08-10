use std::path::PathBuf;

use url::Url;

#[derive(Default, Clone, Debug)]
pub struct GitRepo {}
#[allow(dead_code)]
impl GitRepo {
    pub(crate) fn pull(_repo: &str) -> PathBuf {
        PathBuf::from("./os.sh")
    }
}
#[derive(Default, Clone, Debug)]
pub struct GxShell {}
#[allow(dead_code)]
impl GxShell {
    pub(crate) fn exec(_file: &str, _fun: &str) -> String {
        r#"{"result": true}"#.to_string()
    }
}
#[derive(Default, Clone, Debug)]
pub struct Http {}
impl Http {}
pub fn get_repo_name(url_str: &str) -> Option<String> {
    // 辅助函数：移除.git扩展名
    let remove_git_extension = |name: String| {
        if name.ends_with(".git") {
            name[..name.len() - 4].to_string()
        } else {
            name
        }
    };

    // 辅助函数：判断是否可能是仓库名
    let is_likely_repo_name = |name: &str| {
        // 如果包含常见的仓库名特征（如 .git 在原始URL中），则认为是仓库名
        // 或者名称不是常见的用户名/组织名（如 "user", "org", "team" 等）
        let common_user_names = [
            "user", "users", "org", "orgs", "team", "teams", "group", "groups", "main", "master",
            "tree", "blob",
        ];
        !common_user_names.contains(&name) || name.ends_with(".git")
    };

    // 先尝试处理SSH格式的Git地址
    if url_str.starts_with("git@")
        && let Some(repo_part) = url_str.split(':').next_back()
    {
        if let Some(name) = repo_part.split('/').next_back().map(String::from) {
            if is_likely_repo_name(&name) {
                return Some(remove_git_extension(name));
            }
        }
        return None;
    }

    // 原有HTTP/HTTPS URL处理逻辑
    let url = Url::parse(url_str).ok()?;
    let segments: Vec<_> = url.path_segments()?.collect();

    // 优先查找以.git结尾的路径段
    if let Some(git_segment) = segments.iter().rev().find(|s| s.ends_with(".git")) {
        return Some(remove_git_extension(git_segment.to_string()));
    }

    // 如果没有找到.git段，查找最后一个非空且非常见分支名的段
    if let Some(name) = segments
        .iter()
        .rev()
        .find(|s| !s.is_empty() && is_likely_repo_name(s))
    {
        return Some(remove_git_extension(name.to_string()));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_repo_name_https_github() {
        let url = "https://github.com/user/repo.git";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_https_with_trailing_slash() {
        let url = "https://github.com/user/repo/";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_ssh_format() {
        let url = "git@github.com:user/repo.git";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_ssh_without_git_extension() {
        let url = "git@github.com:user/repo";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_https_gitlab() {
        let url = "https://gitlab.com/user/repo.git";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_bitbucket() {
        let url = "https://bitbucket.org/user/repo.git";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_custom_domain() {
        let url = "https://example.com/user/repo.git";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_no_trailing_slash() {
        let url = "https://github.com/user/repo";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_with_subpath() {
        let url = "https://github.com/user/repo.git/tree/main";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo".to_string()));
    }

    #[test]
    fn test_get_repo_name_invalid_url() {
        let url = "not-a-url";
        let result = get_repo_name(url);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_repo_name_empty_url() {
        let url = "";
        let result = get_repo_name(url);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_repo_name_url_without_repo_name() {
        let url = "https://github.com/user/";
        let result = get_repo_name(url);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_repo_name_ssh_complex() {
        let url = "git@github.com:org/team/repo-name.git";
        let result = get_repo_name(url);
        assert_eq!(result, Some("repo-name".to_string()));
    }

    #[test]
    fn test_git_repo_default_creation() {
        let repo = GitRepo::default();
        // Just test that it doesn't panic and creates a valid instance
        let _ = repo;
    }

    #[test]
    fn test_gx_shell_default_creation() {
        let shell = GxShell::default();
        // Just test that it doesn't panic and creates a valid instance
        let _ = shell;
    }

    #[test]
    fn test_http_default_creation() {
        let http = Http::default();
        // Just test that it doesn't panic and creates a valid instance
        let _ = http;
    }

    #[test]
    fn test_git_repo_clone_and_debug() {
        let repo = GitRepo::default();
        let cloned = repo.clone();
        let debugged = format!("{:?}", cloned);
        assert!(!debugged.is_empty());
    }

    #[test]
    fn test_gx_shell_clone_and_debug() {
        let shell = GxShell::default();
        let cloned = shell.clone();
        let debugged = format!("{:?}", cloned);
        assert!(!debugged.is_empty());
    }

    #[test]
    fn test_http_clone_and_debug() {
        let http = Http::default();
        let cloned = http.clone();
        let debugged = format!("{:?}", cloned);
        assert!(!debugged.is_empty());
    }

    #[test]
    fn test_get_repo_name_edge_cases() {
        let test_cases = vec![
            ("https://github.com/user/repo.name.git", Some("repo.name")),
            ("https://github.com/user/repo_name.git", Some("repo_name")),
            ("https://github.com/user/123repo.git", Some("123repo")),
            ("https://github.com/user/repo-v1.0.git", Some("repo-v1.0")),
        ];

        for (url, expected) in test_cases {
            let result = get_repo_name(url);
            assert_eq!(
                result,
                expected.map(String::from),
                "Failed for URL: {}",
                url
            );
        }
    }
}
