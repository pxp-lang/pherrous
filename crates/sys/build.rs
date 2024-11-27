use std::path::{Path, PathBuf};

use anyhow::Result;

const DEFAULT_EXTENSIONS: &str = "apcu,bcmath,calendar,ctype,curl,dba,dom,exif,fileinfo,filter,gd,iconv,intl,mbregex,mbstring,mysqli,mysqlnd,opcache,openssl,pcntl,pdo,pdo_mysql,pdo_pgsql,pdo_sqlite,pgsql,phar,posix,readline,redis,session,simplexml,sockets,sodium,sqlite3,tokenizer,xml,xmlreader,xmlwriter,xsl,zip,zlib";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/pherrous.c");

    check_conflicting_features()?;

    let spc = download_spc()?.display().to_string();

    download_spc_sources(&spc)?;
    build_spc(&spc)?;

    Ok(())
}

fn download_spc() -> Result<PathBuf> {
    let out_dir = out_dir();

    // Fetch a pre-compiled SPC binary for the current platform.
    exec("curl", &[
        "-fsSL",
        "-o",
        "spc.tgz",
        // FIXME: Add Unix & Windows support here.
        // FIXME: Add support for other architectures too.
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        "https://dl.static-php.dev/static-php-cli/spc-bin/nightly/spc-macos-aarch64.tar.gz"
    ], &out_dir)?;

    // Unpack the SPC binary.
    exec("tar", &[
        "-zxvf",
        "spc.tgz",
    ], &out_dir)?;

    // Remove the tarball.
    std::fs::remove_file(out_dir.join("spc.tgz"))?;

    Ok(out_dir.join("spc"))
}

fn download_spc_sources(spc: &str) -> Result<()> {
    exec(spc, &[
        "download",
        get_spc_version_flag(),
        "--for-extensions",
        DEFAULT_EXTENSIONS,
        "--prefer-pre-built",
    ], &out_dir())?;

    Ok(())
}

fn build_spc(spc: &str) -> Result<()> {
    let mut arguments = Vec::from([
        "build",
        DEFAULT_EXTENSIONS,
        "--build-cli",
    ]);

    if should_build_embed() {
        arguments.push("--build-embed");
    }

    if should_build_zts() {
        arguments.push("--enable-zts");
    }

    exec(spc, &arguments, &out_dir())?;

    Ok(())
}

fn out_dir() -> PathBuf {
    PathBuf::from(std::env::var("OUT_DIR").unwrap())
}

fn check_conflicting_features() -> Result<()> {
    #[cfg(all(feature = "php83", feature = "php84"))]
    compile_error!("You can only build against a single PHP version at a time.");

    Ok(())
}

fn get_spc_version_flag() -> &'static str {
    match true {
        _ if is_php83() => "--with-php=8.3",
        _ if is_php84() => "--with-php=8.4",
        _ => unreachable!()
    }
}

fn should_build_embed() -> bool {
    std::env::var("CARGO_FEATURE_EMBED").is_ok()
}

fn should_build_zts() -> bool {
    std::env::var("CARGO_FEATURE_ZTS").is_ok()
}

fn get_embed_flag() -> &'static str {
    match should_build_embed() {
        true => "--embed",
        false => ""
    }
}

fn get_zts_flag() -> &'static str {
    match should_build_zts() {
        true => "--enable-zts",
        false => ""
    }
}

fn is_php83() -> bool {
    std::env::var("CARGO_FEATURE_PHP83").is_ok()
}

fn is_php84() -> bool {
    std::env::var("CARGO_FEATURE_PHP84").is_ok()
}

fn exec(cmd: &str, args: &[&str], cwd: &Path) -> Result<String> {
    let output = std::process::Command::new(cmd)
        .current_dir(cwd)
        .args(args)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("Failed to execute command: {}", String::from_utf8_lossy(&output.stderr)));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
