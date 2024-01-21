mod builder;
mod builder_option;
mod error;
mod profile;
mod target_browser;
mod target_package;

use builder_option::BuilderOption;
use error::BuildError;
use profile::Profile;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use structopt::StructOpt;
use target_browser::TargetBrowser;

use time::{format_description, OffsetDateTime};

use crate::builder::Builder;

fn main() {
    let option = BuilderOption::from_args();
    println!("{:#?}", option);

    let result = build(option);
    if let Err(error) = result {
        match error {
            BuildError::CargoBuild(ref stderr) | BuildError::WasmBindgenBuild(ref stderr) => {
                // print_error(stderr.to_vec(), error.to_string());
                print_error(stderr.to_owned(), error.to_string());
            }
            _ => {
                println!();
                println!("{}", error.to_string());
                println!();
            }
        }
        return;
    }

    println!();
    println!("build success!");
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    println!(
        "{}",
        OffsetDateTime::now_local()
            .expect("get local time")
            .format(&format)
            .unwrap()
    );
    println!();
    // io::stdout()
    //     .write_all(&output.stdout)
    //     .expect("write stdout");
}

fn build(option: BuilderOption) -> Result<(), BuildError> {
    let BuilderOption {
        package,
        browser,
        release,
    } = option;
    let profile = Profile::from(release);
    browser
        .into_iter()
        .try_for_each(|b| -> Result<(), BuildError> {
            println!("build browser {b} now...");
            package
                .into_iter()
                .try_for_each(|p| -> Result<(), BuildError> {
                    println!("compile package {p} now...");
                    Builder::new(p, b, profile).build()?;
                    Ok(())
                })?;
            copy_static(b, profile)?;
            rename_manifest(b, profile)?;
            remove_unused_manifest(b, profile)?;
            Ok(())
        })?;
    Ok(())
}

fn print_error(stderr: Vec<u8>, message: impl AsRef<str>) {
    println!();
    println!("{}", message.as_ref().to_string());
    println!();
    io::stderr().write_all(&stderr).expect("write_all");
}

fn copy_static(browser: TargetBrowser, profile: Profile) -> io::Result<()> {
    copy_dir_all("../../static", format!("../../dist/{profile}/{browser}"))
}

fn rename_manifest(browser: TargetBrowser, profile: Profile) -> io::Result<()> {
    let version = browser.manifest_version();
    let src_file = format!("../../dist/{profile}/{browser}/manifest_v{version}.json");
    let dst_file = format!("../../dist/{profile}/{browser}/manifest.json");
    fs::rename(src_file, dst_file)
}

fn remove_unused_manifest(browser: TargetBrowser, profile: Profile) -> io::Result<()> {
    let path = format!(
        "../../dist/{}/{}/manifest_v{}.json",
        profile,
        browser,
        browser.unused_manifest_version(),
    );
    match browser {
        TargetBrowser::All => Ok(()),
        TargetBrowser::Chrome | TargetBrowser::Firefox => fs::remove_file(path),
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    fs::read_dir(src)?
        .map(|entry| {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let file_name = entry.file_name();
            if file_type.is_dir() {
                copy_dir_all(entry.path(), dst.as_ref().join(file_name))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(file_name))?;
            }
            Ok(())
        })
        .collect::<io::Result<Vec<()>>>()?;
    // .collect::<Result<Vec<()>, io::Error>>()?;
    Ok(())
}

#[test]
fn should_time() {
    // let format = format_description::parse(
    //     "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour \
    //          sign:mandatory]:[offset_minute]:[offset_second]",
    // )
    // .expect("parse format");
    let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]")
        .expect("parse format");
    println!(
        "{}",
        OffsetDateTime::now_local()
            .expect("get local time")
            .format(&format)
            .expect("format time")
    );
}
