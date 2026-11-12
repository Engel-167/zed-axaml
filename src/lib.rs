//! Zed extension for AXAML, the XAML of Avalonia® apps.
//!
//! Highlighting comes from the tree-sitter XML grammar and the queries in `languages/axaml`.
//! Everything semantic (bindings, x:DataType, types, completion) comes from `axaml-ls`, a .NET
//! language server that loads the project with Roslyn (https://github.com/Engel-167/axaml-ls).
//! This crate only finds or downloads it and decides how to start it.

use std::fs;

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const SERVER_NAME: &str = "axaml-ls";
const SERVER_REPO: &str = "Engel-167/axaml-ls";
/// The framework-dependent build inside each release zip, run as `dotnet AxamlLs.dll`.
const SERVER_DLL: &str = "AxamlLs.dll";

#[derive(Default)]
struct AxamlExtension {
    /// The downloaded server's dll, once found or downloaded in this session.
    cached_dll: Option<String>,
}

impl AxamlExtension {
    fn server_command(&mut self, id: &LanguageServerId, worktree: &zed::Worktree) -> Result<zed::Command> {
        // The project's shell environment, so `dotnet` and DOTNET_ROOT from mise/direnv/etc. are visible.
        let mut env = worktree.shell_env();
        let settings = LspSettings::for_worktree(id.as_ref(), worktree).ok();
        let binary = settings.as_ref().and_then(|s| s.binary.as_ref());

        if let Some(extra) = binary.and_then(|b| b.env.as_ref()) {
            env.extend(extra.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        let args = binary.and_then(|b| b.arguments.clone()).unwrap_or_default();

        // 1. An explicit path in settings: "lsp": { "axaml-ls": { "binary": { "path": "..." } } }
        if let Some(path) = binary.and_then(|b| b.path.clone()) {
            return Ok(zed::Command { command: path, args, env });
        }
        // 2. `axaml-ls` on the PATH (a local build: `mise run install` in the axaml-ls repository).
        if let Some(path) = worktree.which(SERVER_NAME) {
            return Ok(zed::Command { command: path, args, env });
        }
        // 3. The latest release from GitHub, run with the project's dotnet (the server needs the SDK anyway).
        let dotnet = worktree.which("dotnet").ok_or_else(|| {
            "axaml-ls needs the .NET SDK: `dotnet` was not found on the PATH of this project.".to_string()
        })?;
        let dll = self.downloaded_dll(id)?;
        let mut full_args = vec![dll];
        full_args.extend(args);
        Ok(zed::Command { command: dotnet, args: full_args, env })
    }

    /// Absolute path of the release's AxamlLs.dll, downloading the latest release when it is not here yet.
    fn downloaded_dll(&mut self, id: &LanguageServerId) -> Result<String> {
        if let Some(dll) = &self.cached_dll {
            if fs::metadata(dll).is_ok_and(|m| m.is_file()) {
                return Ok(dll.clone());
            }
        }

        zed::set_language_server_installation_status(id, &zed::LanguageServerInstallationStatus::CheckingForUpdate);
        let release = match zed::latest_github_release(
            SERVER_REPO,
            zed::GithubReleaseOptions { require_assets: true, pre_release: false },
        ) {
            Ok(release) => release,
            // Offline or rate-limited: keep using a version downloaded earlier, if there is one.
            Err(error) => {
                zed::set_language_server_installation_status(id, &zed::LanguageServerInstallationStatus::None);
                return self.any_downloaded_dll().ok_or(error);
            }
        };

        let version_dir = format!("{SERVER_NAME}-{}", release.version.trim_start_matches('v'));
        let dll = format!("{version_dir}/{SERVER_DLL}");
        if !fs::metadata(&dll).is_ok_and(|m| m.is_file()) {
            let asset_name = format!("{version_dir}.zip");
            let asset = release
                .assets
                .iter()
                .find(|a| a.name == asset_name)
                .ok_or_else(|| format!("release {} of {SERVER_REPO} has no {asset_name}", release.version))?;

            zed::set_language_server_installation_status(id, &zed::LanguageServerInstallationStatus::Downloading);
            zed::download_file(&asset.download_url, &version_dir, zed::DownloadedFileType::Zip)
                .map_err(|e| format!("failed to download {asset_name}: {e}"))?;

            // Keep only the version just downloaded.
            if let Ok(entries) = fs::read_dir(".") {
                for entry in entries.flatten() {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_with(&format!("{SERVER_NAME}-")) && name != version_dir {
                        fs::remove_dir_all(entry.path()).ok();
                    }
                }
            }
        }
        zed::set_language_server_installation_status(id, &zed::LanguageServerInstallationStatus::None);

        let absolute = absolute(&dll)?;
        self.cached_dll = Some(absolute.clone());
        Ok(absolute)
    }

    /// A server downloaded in an earlier session (used when GitHub cannot be reached).
    fn any_downloaded_dll(&mut self) -> Option<String> {
        let entries = fs::read_dir(".").ok()?;
        let dll = entries.flatten().map(|e| e.path().join(SERVER_DLL)).find(|p| p.is_file())?;
        let absolute = absolute(dll.to_str()?).ok()?;
        self.cached_dll = Some(absolute.clone());
        Some(absolute)
    }
}

/// `dotnet` runs outside the extension's sandbox, so it needs the full path of the dll in the extension's work dir.
fn absolute(relative: &str) -> Result<String> {
    let dir = std::env::current_dir().map_err(|e| format!("no work directory: {e}"))?;
    Ok(dir.join(relative).to_string_lossy().into_owned())
}

impl zed::Extension for AxamlExtension {
    fn new() -> Self {
        AxamlExtension::default()
    }

    fn language_server_command(&mut self, id: &LanguageServerId, worktree: &zed::Worktree) -> Result<zed::Command> {
        self.server_command(id, worktree)
    }

    fn language_server_initialization_options(
        &mut self,
        id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(id.as_ref(), worktree).ok().and_then(|s| s.initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(id.as_ref(), worktree).ok().and_then(|s| s.settings))
    }
}

zed::register_extension!(AxamlExtension);
