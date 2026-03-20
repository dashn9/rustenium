use std::path::{Path, PathBuf};
use std::process::Command;

const VERSION: &str = "146.0.7680.153";
const BASE_URL: &str = "https://storage.googleapis.com/chrome-for-testing-public";

fn platform() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", _) => "linux64",
        ("macos", "aarch64") => "mac-arm64",
        ("macos", _) => "mac-x64",
        ("windows", "x86") => "win32",
        ("windows", _) => "win64",
        (os, arch) => panic!("Unsupported platform: {os}/{arch}"),
    }
}

fn cache_dir(tool: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rustenium-{tool}-{VERSION}-{}", platform()))
}

fn exe_name(tool: &str) -> String {
    if cfg!(windows) { format!("{tool}.exe") } else { tool.to_string() }
}

fn find_exe(dir: &Path, name: &str) -> Option<PathBuf> {
    std::fs::read_dir(dir).ok()?.flatten().find_map(|e| {
        let p = e.path();
        if p.is_dir() {
            find_exe(&p, name)
        } else if p.file_name().is_some_and(|f| f == name) {
            Some(p)
        } else {
            None
        }
    })
}

fn download_zip(url: &str, dest: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let zip = dest.join("download.zip");
    let zip_str = zip.to_str().unwrap();
    let dest_str = dest.to_str().unwrap();

    run("curl", &["-sL", "-o", zip_str, url])?;

    if cfg!(unix) {
        run("unzip", &["-oq", zip_str, "-d", dest_str])?;
    } else {
        run("tar", &["-xf", zip_str, "-C", dest_str])?;
    }

    let _ = std::fs::remove_file(&zip);
    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(cmd).args(args).status()
        .map_err(|e| format!("{cmd} failed: {e}"))?;
    status.success().then_some(()).ok_or_else(|| format!("{cmd} exited with {status}"))
}

fn make_executable(_path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(_path, std::fs::Permissions::from_mode(0o755));
    }
}

fn ensure_tool(tool: &str) -> PathBuf {
    let dir = cache_dir(tool);
    let name = exe_name(tool);

    if let Some(path) = find_exe(&dir, &name) {
        make_executable(&path);
        return path;
    }

    let url = format!("{BASE_URL}/{VERSION}/{}/{tool}-{}.zip", platform(), platform());
    tracing::info!("Downloading {tool} {VERSION} for {} ...", platform());
    download_zip(&url, &dir).unwrap_or_else(|e| panic!("Failed to download {tool}: {e}"));

    let path = find_exe(&dir, &name)
        .unwrap_or_else(|| panic!("{name} not found after extraction in {dir:?}"));
    make_executable(&path);
    tracing::info!("{tool} ready at {path:?}");
    path
}

/// Downloads chromedriver if not already cached. Returns the path to the executable.
pub fn ensure_chromedriver() -> PathBuf {
    ensure_tool("chromedriver")
}

/// Downloads Chrome browser if not already cached. Returns the path to the executable.
pub fn ensure_chrome() -> PathBuf {
    ensure_tool("chrome")
}
