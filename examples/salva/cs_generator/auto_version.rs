use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn update_and_get_version() -> io::Result<String> {
    let file_path = "auto_version.txt";

    if !Path::new(file_path).exists() {
        // If the file doesn't exist, create it with the initial version 0.0.1
        let mut file = File::create(file_path)?;
        file.write_all(b"0.0.1")?;
        println!("Created {} with version 0.0.1", file_path);
        return Ok("0.0.1".to_string());
    } else {
        // If the file exists, read its content
        let mut file = File::open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        // Parse the version and increment the patch (z) part
        let new_version = match increment_version(&content) {
            Ok(version) => version,
            Err(e) => {
                eprintln!("Error parsing version: {}", e);
                return Err(io::Error::new(io::ErrorKind::InvalidData, e));
            }
        };

        // Overwrite the file with the new version
        let mut file = File::create(file_path)?;
        file.write_all(new_version.as_bytes())?;
        println!("Updated {} to version {}", file_path, new_version);
        Ok(new_version)
    }
}

pub fn get_version() -> io::Result<String> {
    let file_path = "auto_version.txt";

    if !Path::new(file_path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Version file not found"));
    }

    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn increment_version(version: &str) -> Result<String, &'static str> {
    let parts: Vec<&str> = version.trim().split('.').collect();
    if parts.len() != 3 {
        return Err("Version format is invalid. Expected x.y.z");
    }

    let major: u32 = parts[0].parse().map_err(|_| "Invalid major version")?;
    let minor: u32 = parts[1].parse().map_err(|_| "Invalid minor version")?;
    let mut patch: u32 = parts[2].parse().map_err(|_| "Invalid patch version")?;

    patch += 1; // Increment the patch part

    Ok(format!("{}.{}.{}", major, minor, patch))
}

pub fn copy_with_version(source_dir: &str, output_dir: &str, file_name: &str, version: &str, diff_source_name: Option<&str>) -> io::Result<String> {
    let dll_file = format!("{}.dll", file_name);
    let so_file = format!("{}.so", file_name);

    let target_base_name = format!("{}_{}", file_name, version);

    for file_name in &[dll_file, so_file] {
        let mut source_path = Path::new(source_dir).join(file_name);
        if let Some(source_file_name) = diff_source_name {
            if file_name.contains("dll") {
                continue;
            }
            source_path = Path::new(source_dir).join(format!("{}.so", source_file_name));
        }
        if !source_path.exists() {
            println!("File {} does not exist in the source directory ({})", file_name, source_path.display());
            continue;
        }

        if let Some(extension) = Path::new(file_name).extension() {
            let output_path = Path::new(output_dir).join(format!("{}.{}", target_base_name, extension.to_string_lossy()));
            fs::copy(&source_path, &output_path)?;
            println!("Copied {} to {}", source_path.display(), output_path.display());
        }
    }

    Ok(target_base_name)
}
