use std::collections::HashSet;
use std::env;
use std::path::Path;
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, Result};

const PACKAGE_NAME: &str = "@tact-lang/tact-language-server";

struct TactExtension {
    installed: HashSet<String>,
}

impl TactExtension {
    fn install_package_if_needed(
        &mut self,
        id: &zed::LanguageServerId,
        package_name: &str,
    ) -> Result<()> {
        let installed_version = zed::npm_package_installed_version(package_name)?;

        // If package is already installed in this session, then we won't reinstall it
        if installed_version.is_some() && self.installed.contains(package_name) {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let latest_version = zed::npm_package_latest_version(package_name)?;

        if installed_version.as_ref() != Some(&latest_version) {
            println!("Installing {package_name}@{latest_version}...");

            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            if let Err(error) = zed::npm_install_package(package_name, &latest_version) {
                // If installation failed, but we don't want to error but rather reuse existing version
                if installed_version.is_none() {
                    Err(error)?;
                }
            }
        } else {
            println!("Found {package_name}@{latest_version} installed");
        }

        self.installed.insert(package_name.into());
        Ok(())
    }
}

impl zed::Extension for TactExtension {
    fn new() -> Self {
        Self {
            installed: HashSet::new(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        self.install_package_if_needed(language_server_id, PACKAGE_NAME)?;

        let lsp_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        // If the user has specified a binary in their LSP settings,
        // that takes precedence.
        if let Some(binary_settings) = lsp_settings.binary {
            if let Some(path) = binary_settings.path {
                println!(
                    "Using user provided path for tact-language-server: {}",
                    path
                );
                return Ok(zed::Command {
                    command: path,
                    args: binary_settings.arguments.unwrap_or_else(Vec::new),
                    env: worktree.shell_env(),
                });
            }
        }

        let lsp_path = zed_ext::sanitize_windows_path(env::current_dir().unwrap())
            .join("node_modules")
            .join(PACKAGE_NAME)
            .join("server.js")
            .to_string_lossy()
            .to_string();
        Path::new(lsp_path.as_str())
            .try_exists()
            .expect("tact-languge-server not found.");

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![lsp_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TactExtension);

/// Extensions to the Zed extension API that have not yet stabilized.
mod zed_ext {
    /// Sanitizes the given path to remove the leading `/` on Windows.
    ///
    /// On macOS and Linux this is a no-op.
    ///
    /// This is a workaround for https://github.com/bytecodealliance/wasmtime/issues/10415.
    pub fn sanitize_windows_path(path: std::path::PathBuf) -> std::path::PathBuf {
        use zed_extension_api::{Os, current_platform};

        let (os, _arch) = current_platform();
        match os {
            Os::Mac | Os::Linux => path,
            Os::Windows => path
                .to_string_lossy()
                .to_string()
                .trim_start_matches('/')
                .into(),
        }
    }
}
