use std::path::{Path, PathBuf};
use std::process::Command;

// ── Chrome ───────────────────────────────────────────────────────────────────

const CHROME_VERSION: &str = "146.0.7680.153";
const CHROME_BASE_URL: &str = "https://storage.googleapis.com/chrome-for-testing-public";

fn chrome_platform() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", _) => "linux64",
        ("macos", "aarch64") => "mac-arm64",
        ("macos", _) => "mac-x64",
        ("windows", "x86") => "win32",
        ("windows", _) => "win64",
        (os, arch) => panic!("Unsupported platform: {os}/{arch}"),
    }
}

/// Downloads chromedriver if not already cached. Returns the path to the executable.
pub fn ensure_chromedriver() -> PathBuf {
    let dir = cache_dir("chromedriver", CHROME_VERSION, chrome_platform());
    let name = exe_name("chromedriver");
    if let Some(path) = find_exe(&dir, &name) {
        make_executable(&path);
        return path;
    }

    let url = format!(
        "{CHROME_BASE_URL}/{CHROME_VERSION}/{0}/chromedriver-{0}.zip",
        chrome_platform()
    );
    tracing::info!("Downloading chromedriver {CHROME_VERSION} for {} ...", chrome_platform());
    download_and_extract(&url, &dir)
        .unwrap_or_else(|e| panic!("Failed to download chromedriver: {e}"));

    let path = find_exe(&dir, &name)
        .unwrap_or_else(|| panic!("{name} not found after extraction in {dir:?}"));
    make_executable(&path);
    tracing::info!("chromedriver ready at {path:?}");
    path
}

/// Downloads Chrome browser if not already cached. Returns the path to the executable.
pub fn ensure_chrome() -> PathBuf {
    let dir = cache_dir("chrome", CHROME_VERSION, chrome_platform());
    let name = exe_name("chrome");
    if let Some(path) = find_exe(&dir, &name) {
        make_executable(&path);
        return path;
    }

    let url = format!(
        "{CHROME_BASE_URL}/{CHROME_VERSION}/{0}/chrome-{0}.zip",
        chrome_platform()
    );
    tracing::info!("Downloading Chrome {CHROME_VERSION} for {} ...", chrome_platform());
    download_and_extract(&url, &dir)
        .unwrap_or_else(|e| panic!("Failed to download Chrome: {e}"));

    let path = find_exe(&dir, &name)
        .unwrap_or_else(|| panic!("{name} not found after extraction in {dir:?}"));
    make_executable(&path);
    tracing::info!("Chrome ready at {path:?}");
    path
}

// ── Geckodriver ──────────────────────────────────────────────────────────────

const GECKODRIVER_VERSION: &str = "0.36.0";
const GECKODRIVER_BASE_URL: &str = "https://github.com/mozilla/geckodriver/releases/download";

/// Returns (platform_name, archive_extension) for geckodriver downloads.
fn geckodriver_platform() -> (&'static str, &'static str) {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "aarch64") => ("linux-aarch64", "tar.gz"),
        ("linux", "x86") => ("linux32", "tar.gz"),
        ("linux", _) => ("linux64", "tar.gz"),
        ("macos", "aarch64") => ("macos-aarch64", "tar.gz"),
        ("macos", _) => ("macos", "tar.gz"),
        ("windows", "x86") => ("win32", "zip"),
        ("windows", "aarch64") => ("win-aarch64", "zip"),
        ("windows", _) => ("win64", "zip"),
        (os, arch) => panic!("Unsupported platform for geckodriver: {os}/{arch}"),
    }
}

/// Downloads geckodriver if not already cached. Returns the path to the executable.
pub fn ensure_geckodriver() -> PathBuf {
    let (plat, ext) = geckodriver_platform();
    let dir = cache_dir("geckodriver", GECKODRIVER_VERSION, plat);
    let name = exe_name("geckodriver");
    if let Some(path) = find_exe(&dir, &name) {
        make_executable(&path);
        return path;
    }

    let url = format!(
        "{GECKODRIVER_BASE_URL}/v{GECKODRIVER_VERSION}/geckodriver-v{GECKODRIVER_VERSION}-{plat}.{ext}"
    );
    tracing::info!("Downloading geckodriver {GECKODRIVER_VERSION} for {plat} ...");
    download_and_extract(&url, &dir)
        .unwrap_or_else(|e| panic!("Failed to download geckodriver: {e}"));

    let path = find_exe(&dir, &name)
        .unwrap_or_else(|| panic!("{name} not found after extraction in {dir:?}"));
    make_executable(&path);
    tracing::info!("geckodriver ready at {path:?}");
    path
}

// ── Firefox ──────────────────────────────────────────────────────────────────

const FIREFOX_VERSION: &str = "149.0";
const FIREFOX_BASE_URL: &str = "https://ftp.mozilla.org/pub/firefox/releases";

/// Downloads Firefox if not already cached. Returns the path to the firefox executable.
pub fn ensure_firefox() -> PathBuf {
    let plat = firefox_platform();
    let dir = cache_dir("firefox", FIREFOX_VERSION, plat);
    let name = exe_name("firefox");
    if let Some(path) = find_exe(&dir, &name) {
        make_executable(&path);
        return path;
    }

    tracing::info!("Downloading Firefox {FIREFOX_VERSION} for {plat} ...");
    download_firefox(&dir)
        .unwrap_or_else(|e| panic!("Failed to download Firefox: {e}"));

    let path = find_exe(&dir, &name)
        .unwrap_or_else(|| panic!("{name} not found after extraction in {dir:?}"));
    make_executable(&path);
    tracing::info!("Firefox ready at {path:?}");
    path
}

fn firefox_platform() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "aarch64") => "linux-aarch64",
        ("linux", _) => "linux-x86_64",
        ("macos", _) => "mac",
        ("windows", "x86") => "win32",
        ("windows", _) => "win64",
        (os, arch) => panic!("Unsupported platform for Firefox: {os}/{arch}"),
    }
}

fn download_firefox(dest: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;

    match std::env::consts::OS {
        "linux" => {
            let plat = firefox_platform();
            let url = format!(
                "{FIREFOX_BASE_URL}/{FIREFOX_VERSION}/{plat}/en-US/firefox-{FIREFOX_VERSION}.tar.xz"
            );
            download_and_extract(&url, dest)
        }
        "windows" => {
            let plat = firefox_platform();
            let url = format!(
                "{FIREFOX_BASE_URL}/{FIREFOX_VERSION}/{plat}/en-US/Firefox%20Setup%20{FIREFOX_VERSION}.exe"
            );
            let exe = dest.join("firefox-setup.exe");
            let exe_str = exe.to_str().unwrap().replace('/', "\\");
            run("curl", &["-sL", "-o", &exe_str, &url])?;

            // Firefox installer supports /S (silent) and /D= (install directory)
            // /D= must be the last arg with no quotes and use backslashes
            let install_dir = dest.to_str().unwrap().replace('/', "\\");
            run(
                &exe_str,
                &["/S", &format!("/D={install_dir}")],
            )?;

            let _ = std::fs::remove_file(&exe);
            Ok(())
        }
        "macos" => {
            let url = format!(
                "{FIREFOX_BASE_URL}/{FIREFOX_VERSION}/mac/en-US/Firefox%20{FIREFOX_VERSION}.dmg"
            );
            let dmg = dest.join("firefox.dmg");
            let dmg_str = dmg.to_str().unwrap();
            run("curl", &["-sL", "-o", dmg_str, &url])?;

            let mount_point = dest.join("_dmg_mount");
            let mount_str = mount_point.to_str().unwrap();
            std::fs::create_dir_all(&mount_point).map_err(|e| e.to_string())?;

            run(
                "hdiutil",
                &["attach", "-nobrowse", "-readonly", "-mountpoint", mount_str, dmg_str],
            )?;

            let copy_result = run(
                "cp",
                &["-R", &format!("{mount_str}/Firefox.app"), dest.to_str().unwrap()],
            );

            // Always attempt to detach, even if copy failed
            let _ = run("hdiutil", &["detach", mount_str]);
            let _ = std::fs::remove_file(&dmg);
            let _ = std::fs::remove_dir_all(&mount_point);

            copy_result?;
            Ok(())
        }
        other => Err(format!("Unsupported OS for Firefox download: {other}")),
    }
}

/// Attempts to find the system-installed Firefox executable.
pub fn find_system_firefox() -> Option<PathBuf> {
    if cfg!(windows) {
        let candidates = [
            r"C:\Program Files\Mozilla Firefox\firefox.exe",
            r"C:\Program Files (x86)\Mozilla Firefox\firefox.exe",
        ];
        for path in &candidates {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
    } else if cfg!(target_os = "macos") {
        let p = PathBuf::from("/Applications/Firefox.app/Contents/MacOS/firefox");
        if p.exists() {
            return Some(p);
        }
    } else {
        if let Ok(output) = Command::new("which").arg("firefox").output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !path.is_empty() {
                    return Some(PathBuf::from(path));
                }
            }
        }
    }
    None
}

// ── Shared helpers ───────────────────────────────────────────────────────────

fn cache_dir(tool: &str, version: &str, platform: &str) -> PathBuf {
    std::env::temp_dir().join(format!("rustenium-{tool}-{version}-{platform}"))
}

fn exe_name(tool: &str) -> String {
    if cfg!(windows) {
        format!("{tool}.exe")
    } else {
        tool.to_string()
    }
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

fn download_and_extract(url: &str, dest: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let archive = dest.join("download_archive");
    let archive_str = archive.to_str().unwrap();
    let dest_str = dest.to_str().unwrap();

    run("curl", &["-sL", "-o", archive_str, url])?;

    if url.ends_with(".tar.xz") {
        run("tar", &["-xJf", archive_str, "-C", dest_str])?;
    } else if url.ends_with(".tar.gz") {
        run("tar", &["-xzf", archive_str, "-C", dest_str])?;
    } else if url.ends_with(".zip") {
        if cfg!(unix) {
            run("unzip", &["-oq", archive_str, "-d", dest_str])?;
        } else {
            run("tar", &["-xf", archive_str, "-C", dest_str])?;
        }
    } else {
        return Err(format!("Unknown archive format: {url}"));
    }

    let _ = std::fs::remove_file(&archive);
    Ok(())
}

fn run(cmd: &str, args: &[&str]) -> Result<(), String> {
    tracing::debug!("Running: {cmd} {}", args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .status()
        .map_err(|e| format!("{cmd} failed: {e}"))?;
    status
        .success()
        .then_some(())
        .ok_or_else(|| format!("{cmd} exited with {status}"))
}

fn make_executable(_path: &Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(_path, std::fs::Permissions::from_mode(0o755));
    }
}
