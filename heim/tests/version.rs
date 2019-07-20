#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

// `README.md` in the `heim/` dir is a symlink,
// so this test fails in Windows. It is okay to run it at least once in CI.
#[cfg(unix)]
#[test]
fn test_readme_badges() {
    version_sync::assert_contains_regex!(
        "README.md",
        "https://deps.rs/crate/{name}/{version}/status.svg"
    );
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
