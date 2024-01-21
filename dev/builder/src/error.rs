use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("unknown package")]
    UnknownPackage,

    #[error("unknown target")]
    UnknownBrowser,

    #[error("failed to cargo build")]
    CargoBuild(Vec<u8>),

    #[error("failed to wasm-bindgen build")]
    WasmBindgenBuild(Vec<u8>),

    #[error("io error: {0}")]
    IO(#[from] io::Error),
}
