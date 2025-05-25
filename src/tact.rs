use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

struct TactExtension;

impl zed::Extension for TactExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        // If the user has specified a binary in their LSP settings,
        // that takes precedence.
        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path {
                return Ok(zed::Command {
                    command: path,
                    args: binary_settings.arguments.unwrap_or_else(Vec::new),
                    env: worktree.shell_env(),
                });
            }
        }

        let path = worktree
            .which("tact-language-server")
            .ok_or_else(|| "tact-language-server must be installed via npm".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["--stdio".to_string()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(TactExtension);
