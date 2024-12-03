use zed_extension_api as zed;
use std::fmt::Write;

struct ToStringExtension;

impl zed::Extension for ToStringExtension {
    fn new() -> Self {
        ToStringExtension
    }

    fn run_slash_command(
        &self,
        command: zed::SlashCommand,
        _args: Vec<String>,
        _worktree: Option<&zed::Worktree>,
    ) -> zed::Result<zed::SlashCommandOutput> {
        match command.name.as_str() {
            "generate_tostring" => {
                // Log or return a message about the limitation
                Err("Unable to directly modify the current document. This feature requires additional API support.".into())
            },
            _ => Err("Unknown command.".into())
        }
    }
}

/// Adds `toString` methods to TypeScript classes
fn add_tostring_methods(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut modified_content = String::new();
    let mut in_class = false;
    let mut class_name = String::new();
    let mut class_body_indent = String::new();

    for line in content.lines() {
        modified_content.push_str(line);
        modified_content.push('\n');

        let trimmed_line = line.trim();
        if trimmed_line.starts_with("class ") {
            in_class = true;
            // Extract class name and determine indentation
            let parts: Vec<&str> = line.split("class").collect();
            class_body_indent = parts[0].to_string();

            class_name = trimmed_line
                .replace("class", "")
                .trim()
                .split_whitespace()
                .next()
                .unwrap_or("UnknownClass")
                .to_string();
        } else if in_class && trimmed_line.starts_with('}') {
            in_class = false;

            // Generate toString method with proper indentation
            let tostring_method = format!(
                r#"{class_body_indent}    toString() {{
{class_body_indent}        return `{class_name} {{ ` + Object.entries(this)
{class_body_indent}            .map(([k, v]) => `${{k}}: ${{JSON.stringify(v)}}`)
{class_body_indent}            .join(", ") + " }}";
{class_body_indent}    }}"#
            );

            modified_content.push_str(&tostring_method);
            modified_content.push('\n');
        }
    }

    Ok(modified_content)
}

// Register the extension
zed::register_extension!(ToStringExtension);
