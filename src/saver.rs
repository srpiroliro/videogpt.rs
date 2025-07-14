use anyhow::Context;
use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub struct SaverConfig {
    pub custom_name: Option<String>,
    pub url: Option<String>,
    pub folder: Option<String>,
}

fn prepare_folder(folder: &str) -> anyhow::Result<String> {
    if !Path::new(folder).exists() {
        fs::create_dir_all(folder)
            .with_context(|| format!("Failed to create directory: {}", folder))?;
    }

    Ok(folder.to_string())
}

pub fn save(config: SaverConfig, content: String) -> anyhow::Result<()> {
    let mut output = content;

    let filename: String = match config.custom_name.clone() {
        Some(name) => name,
        None => {
            let name = output
                .lines()
                .next()
                .unwrap_or("")
                .replace("# ", "")
                .replace(" ", "-")
                .to_lowercase();

            if Path::new(&name).exists() || name.len() < 1 {
                let id = Uuid::new_v4();

                format!("{id}.md")
            } else {
                format!("{name}.md")
            }
        }
    };

    if let Some(url) = config.url {
        output = format!("Video URL: {}\n\n\n{}", url, output)
    }

    let path = match config.folder {
        Some(folders) => {
            let _ = prepare_folder(&folders)?;
            Path::new(&folders).join(filename)
        }
        None => PathBuf::from(&filename),
    };

    let result = fs::write(&path, output).with_context(|| format!("Something went wrong!"));

    println!("Saved at: {}", path.display());

    result
}
