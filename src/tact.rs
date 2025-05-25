// use zed::lsp::{Symbol, SymbolKind};
// use zed::{CodeLabel, CodeLabelSpan};
// use zed_extension_api::settings::LspSettings;
// use zed_extension_api::{self as zed, Result};
use zed_extension_api::{self as zed};

struct TactExtension;

impl zed::Extension for TactExtension {
    fn new() -> Self {
        Self
    }

    // fn language_server_command(
    //     &mut self,
    //     language_server_id: &zed::LanguageServerId,
    //     worktree: &zed::Worktree,
    // ) -> Result<zed::Command> {
    //     let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

    //     // If the user has specified a binary in their LSP settings,
    //     // that takes precedence.
    //     if let Some(binary_settings) = lsp_settings.binary {
    //         if let Some(path) = binary_settings.path {
    //             return Ok(zed::Command {
    //                 command: path,
    //                 args: binary_settings.arguments.unwrap_or_else(Vec::new),
    //                 env: worktree.shell_env(),
    //             });
    //         }
    //     }

    //     let path = worktree
    //         .which("tact-language-server")
    //         .ok_or_else(|| "tact-language-server must be installed via npm".to_string())?;

    //     Ok(zed::Command {
    //         command: path,
    //         args: vec!["--stdio".to_string()],
    //         env: worktree.shell_env(),
    //     })
    // }

    // fn label_for_symbol(
    //     &self,
    //     _language_server_id: &zed::LanguageServerId,
    //     symbol: Symbol,
    // ) -> Option<CodeLabel> {
    //     let name = &symbol.name;

    //     let (code, display_range, filter_range) = match symbol.kind {
    //         SymbolKind::Struct => {
    //             let data_decl = "data ";
    //             let code = format!("{data_decl}{name} = A");
    //             let display_range = 0..data_decl.len() + name.len();
    //             let filter_range = data_decl.len()..display_range.end;
    //             (code, display_range, filter_range)
    //         }
    //         SymbolKind::Constructor => {
    //             let data_decl = "data A = ";
    //             let code = format!("{data_decl}{name}");
    //             let display_range = data_decl.len()..data_decl.len() + name.len();
    //             let filter_range = 0..name.len();
    //             (code, display_range, filter_range)
    //         }
    //         SymbolKind::Variable => {
    //             let code = format!("{name} :: T");
    //             let display_range = 0..name.len();
    //             let filter_range = 0..name.len();
    //             (code, display_range, filter_range)
    //         }
    //         _ => return None,
    //     };

    //     Some(CodeLabel {
    //         spans: vec![CodeLabelSpan::code_range(display_range)],
    //         filter_range: filter_range.into(),
    //         code,
    //     })
    // }
}

zed::register_extension!(TactExtension);
