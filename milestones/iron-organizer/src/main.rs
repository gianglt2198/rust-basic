use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::{self, read_dir},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple file organizer")]
struct Args {
    path: PathBuf,
}

fn get_folder_name(ext: &str) -> &str {
    match ext.to_lowercase().as_str() {
        "jpg" | "jpeg" | "png" | "gif" => "Images",
        "pdf" | "docx" | "txt" => "Documents",
        "mp3" | "wav" => "Music",
        "mp4" | "mkv" => "Video",
        _ => "Others",
    }
}

fn main() -> Result<()> {
    // 1. Parse the arguments
    // 2. Check if the path exists and is a directory
    // 3. Print "Organizing: [path]" if it's valid
    let args = Args::parse();

    if !args.path.exists() {
        anyhow::bail!("Error: the path {:?} does not exist.", args.path);
    }

    if !args.path.is_dir() {
        anyhow::bail!("Error: the path {:?} is not a directory.", args.path);
    }

    for entry_result in read_dir(&args.path)? {
        let entry = entry_result?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        // 1. Get extension, fallback to "Others" if none exists
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("none");

        let folder_name = get_folder_name(extension);

        // 2. Define the target directory (e.g., path/to/folder/Images)
        let target_dir = args.path.join(folder_name);

        // 3. Create the directory if it doesn't exist
        fs::create_dir_all(&target_dir)
            .with_context(|| format!("Failed to create directory {:?}", target_dir))?;

        // 4. Define the target file path
        let file_name = path.file_name().context("Could not get file name")?;

        let dest_path = target_dir.join(file_name);

        // 5. Move the file
        println!("Moving {:?} to {:?}", file_name, dest_path);
        fs::rename(&path, &dest_path)
            .with_context(|| format!("Failed to move {:?} to {:?}", path, dest_path))?;
    }

    println!("Done! Organization complete.");
    Ok(())
}
