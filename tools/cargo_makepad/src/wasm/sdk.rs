use crate::makepad_shell::*;

const WASM_BINDGEN_CLI_VERSION: &str = "0.2.104";

pub fn install_cli_tools() -> Result<(), String> {
    let cwd = std::env::current_dir()
        .map_err(|e| format!("Unable to determine current directory: {:?}", e))?;
    println!("Installing wasm CLI tools (wasm-bindgen-cli, wasm-split-cli)");
    shell_env(
        &[],
        &cwd,
        "cargo",
        &[
            "install",
            "--locked",
            "wasm-bindgen-cli",
            "--version",
            WASM_BINDGEN_CLI_VERSION,
        ],
    )?;
    shell_env(&[], &cwd, "cargo", &["install", "--locked", "wasm-split-cli"])?;
    Ok(())
}

pub fn rustup_toolchain_install() -> Result<(), String> {
    println!("Installing Rust toolchains for wasm");
    /*
    shell_env(&[],&std::env::current_dir().unwrap(), "rustup", &[
        "update",
    ]) ?;*/
    shell_env(
        &[],
        &std::env::current_dir().unwrap(),
        "rustup",
        &["install", "nightly"],
    )?;
    shell_env(
        &[],
        &std::env::current_dir().unwrap(),
        "rustup",
        &[
            "target",
            "add",
            "wasm32-unknown-unknown",
            "--toolchain",
            "nightly",
        ],
    )?;
    shell_env(
        &[],
        &std::env::current_dir().unwrap(),
        "rustup",
        &[
            "target",
            "add",
            "wasm32-unknown-unknown",
            "--toolchain",
            "nightly",
        ],
    )?;
    shell_env(
        &[],
        &std::env::current_dir().unwrap(),
        "rustup",
        &[
            "component",
            "add",
            "rust-std",
            "--toolchain",
            "nightly",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    shell_env(
        &[],
        &std::env::current_dir().unwrap(),
        "rustup",
        &[
            "component",
            "add",
            "rust-src",
            "--toolchain",
            "nightly",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;

    install_cli_tools()?;

    Ok(())
}

//MAKEPAD=lines RUSTFLAGS="-C codegen-units=1 -C target-feature=+atomics,+bulk-memory,+mutable-globals -C link-arg=--export=__stack_pointer -C opt-level=z" cargo +nightly build $1 $2 --target=wasm32-unknown-unknown --release -Z build-std=panic_abort,std
