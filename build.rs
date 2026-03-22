use std::{fs, io::Write, process::Command};

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Package {
    authors: Vec<String>,
    name: String,
    version: String,
    description: String,
    metadata: Metadata,
}

#[derive(Deserialize)]
struct CargoConfig {
    package: Package,
}

#[derive(Deserialize)]
struct Metadata {
    magic_mount_rs: Update,
}

#[derive(Deserialize)]
struct Update {
    update: String,
    name: String,
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=.git");

    let toml = fs::read_to_string("Cargo.toml")?;
    let data: CargoConfig = toml::from_str(&toml)?;

    gen_module_prop(&data)?;

    Ok(())
}

fn cal_git_code() -> Result<i32> {
    Ok(String::from_utf8(
        Command::new("git")
            .args(["rev-list", "--count", "HEAD"])
            .output()?
            .stdout,
    )?
    .trim()
    .parse::<i32>()?)
}

fn gen_module_prop(data: &CargoConfig) -> Result<()> {
    let package = &data.package;
    let id = package.name.replace('-', "_");
    let version_code = cal_git_code()? + 300000;
    let authors = &package.authors;
    let mut author = String::new();
    let mut conut = 0;
    for a in authors {
        conut += 1;
        if conut > 1 {
            author += &format!("& {a} ");
        } else {
            author += &format!("{a} ");
        }
    }
    let author = author.trim();
    let version = format!("{}-{}", package.version, cal_git_code()?);

    let mut file = fs::OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("module/module.prop")?;

    writeln!(file, "id={id}")?;
    writeln!(file, "name={}", package.metadata.magic_mount_rs.name)?;
    writeln!(file, "version=v{}", version.trim())?;
    writeln!(file, "versionCode={version_code}")?;
    writeln!(file, "author={author}")?;
    writeln!(
        file,
        "updateJson={}",
        package.metadata.magic_mount_rs.update
    )?;
    writeln!(file, "description={}", package.description)?;
    writeln!(file, "actionIcon=launcher.png")?;
    writeln!(file, "metamodule=1")?;
    Ok(())
}
