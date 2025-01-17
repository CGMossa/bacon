pub static DEFAULT_PACKAGE_CONFIG: &str = r#"
# This is a configuration file for the bacon tool
# More info at https://github.com/Canop/bacon

default_job = "check"

[jobs]

[jobs.check]
command = ["cargo", "check", "--tests", "--color", "always"]
need_stdout = false

[jobs.light]
command = ["cargo", "check", "--color", "always"]
need_stdout = false

[jobs.clippy]
command = ["cargo", "clippy", "--color", "always"]
need_stdout = false

[jobs.test]
command = ["cargo", "test", "--color", "always"]
need_stdout = true

"#;
