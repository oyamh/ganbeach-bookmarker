use crate::profile::Profile;
use crate::target_package::TargetPackage;
use crate::{error::BuildError, target_browser::TargetBrowser};
use std::process::Command;

pub struct Builder {
    package: TargetPackage,
    browser: TargetBrowser,
    profile: Profile,
}

impl Builder {
    pub fn new(package: TargetPackage, browser: TargetBrowser, profile: Profile) -> Self {
        Self {
            package,
            browser,
            profile,
        }
    }

    pub fn build(&self) -> Result<(), BuildError> {
        self.build_target()?;
        self.build_wasm_bindgen()?;
        Ok(())
    }

    fn build_target(&self) -> Result<(), BuildError> {
        let src_path = format!("../../src/{}", self.package);
        let mut binding = Command::new("cargo");
        let command = binding
            .arg("build")
            .args(["--target", "wasm32-unknown-unknown"])
            .arg("--lib")
            .args(["--features", self.browser.into()]);
        if self.profile.is_release() {
            command.arg("--release");
        };
        let output = command.current_dir(src_path).output().expect("cargo build");
        if output.status.success() {
            Ok(())
        } else {
            Err(BuildError::CargoBuild(output.stderr))
        }
    }

    fn build_wasm_bindgen(&self) -> Result<(), BuildError> {
        let target = format!(
            "target/wasm32-unknown-unknown/{}/{}.wasm",
            self.profile, self.package
        );
        let mut binding = Command::new("wasm-bindgen");
        let command = binding
            .arg(target)
            .args(["--target", "web"])
            .arg("--no-typescript");
        let dist_path = format!(
            "dist/{}/{}/wasm/{}",
            self.profile, self.browser, self.package
        );
        if self.profile.is_release() {
            command.args(["--out-dir", &dist_path]);
        } else {
            command
                .arg("--debug")
                .arg("--keep-debug")
                .args(["--out-dir", &dist_path]);
        }
        let output = command
            .current_dir("../../")
            .output()
            .expect("wasm-bindgen");
        if output.status.success() {
            Ok(())
        } else {
            Err(BuildError::WasmBindgenBuild(output.stderr))
        }
    }
}
