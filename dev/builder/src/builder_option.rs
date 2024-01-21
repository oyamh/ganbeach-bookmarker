use crate::{
    target_browser::{TargetBrowser, BROWSER_ALL},
    target_package::{TargetPackage, PACKAGE_ALL},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "builder")]
pub struct BuilderOption {
    #[structopt(short, long, default_value = PACKAGE_ALL)]
    pub package: TargetPackage,

    #[structopt(short, long, default_value = BROWSER_ALL)]
    pub browser: TargetBrowser,

    #[structopt(short, long)]
    pub release: bool,
}
