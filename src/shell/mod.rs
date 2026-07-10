pub mod bash;
pub mod zsh;

pub fn get_snippet(shell: &str) -> Result<String, String> {
    match shell {
        "zsh" => Ok(zsh::SNIPPET.to_string()),
        "bash" => Ok(bash::SNIPPET.to_string()),
        _ => Err(format!(
            "unsupported shell: {}. Supported: zsh, bash",
            shell
        )),
    }
}
