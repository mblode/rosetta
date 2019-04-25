use std::fs::{canonicalize, create_dir};
use std::path::Path;
use std::fs::File;
use std::io::Write;

use exitfailure::ExitFailure;
use failure::ResultExt;

const CONFIG: &str = r#"
baseURL = "http://example.org/"
languageCode = "en-us"
title = "My New Rusta Site"
"#;

pub fn create_file(path: &Path, content: &str) -> Result<(), ExitFailure> {
    let mut file = File::create(&path)
        .with_context(|_| format!("Failed to create file"))?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub fn new_site (name: &str) -> Result<(), ExitFailure> {
    let path = Path::new(name);

    create_dir(path)?;

    let config = CONFIG.trim_start();
    create_file(&path.join("config.toml"), &config)?;
    create_dir(path.join("archetypes"))?;
    create_dir(path.join("content"))?;
    create_dir(path.join("data"))?;
    create_dir(path.join("layouts"))?;
    create_dir(path.join("static"))?;
    create_dir(path.join("themes"))?;

    println!("Congratulations! Your new Rusta site is created in {:?}", canonicalize(path).unwrap());

    Ok(())
}
