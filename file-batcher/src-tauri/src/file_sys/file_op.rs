/**
 * 文件操作
 */

use std::path::Path;
use std::fs;
use rayon::prelude::*;
use std::io::{BufReader, BufWriter, Read, Write};
use std::fs::File;

#[allow(dead_code)]
pub struct FileOp;

#[allow(dead_code)]
impl FileOp {
    pub fn copy_file(source: &str, target: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let path = Path::new(source);
        if !path.exists() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")));
        }
        let target_path = Path::new(target);
        if target_path.exists() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists")));
        }
        fs::copy(source, target)?;
        Ok(())
    }

    pub fn copy_large_file(source: &str, target: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let mut reader = BufReader::new(File::open(source)?);
        let mut writer = BufWriter::new(File::create(target)?);

        let mut buffer = vec![0u8; 1024 * 1024];
        loop {
            let n = reader.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            writer.write_all(&buffer[..n])?;
        }
        Ok(())
    }

    pub fn paste_files(sources: Vec<String>, target: &str, is_cut: bool) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let target_path = Path::new(target);
        if !target_path.exists() {
            fs::create_dir_all(target_path)?;
        }

        sources.par_iter().try_for_each(|source| -> std::io::Result<()> {
            let source_path = Path::new(source);
            let file_name = source_path.file_name().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid file path")
            })?;
            let dest_file_path = target_path.join(file_name);
            if source_path.is_dir() {
                Self::copy_dir_all(source_path, &dest_file_path)?;
                if is_cut {
                    fs::remove_dir_all(source_path)?;
                }
            } else {
                fs::copy(source_path, dest_file_path)?;
                if is_cut {
                    fs::remove_file(source_path)?;
                }
            }

            Ok(())
        })?;
        Ok(())
    }

    fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
        if !dst.exists() {
            fs::create_dir_all(dst)?;
        }
        let entries = fs::read_dir(src)?.collect::<Result<Vec<_>, _>>()?;

        entries.into_par_iter().try_for_each(|entry| -> std::io::Result<()> {
            let file_type = entry.file_type()?;
            let dest_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                Self::copy_dir_all(&entry.path(), &dest_path)?;
            } else {
                fs::copy(entry.path(), dest_path)?;
            }
            Ok(())
        })?;

        Ok(())
    }

    pub fn delete_files(sources: Vec<String>) -> Result<(), Box<dyn std::error::Error + 'static>> {
        sources.par_iter().try_for_each(|source| -> std::io::Result<()> {
            let source_path = Path::new(source);
            if source_path.is_dir() {
                match trash::delete_all(source_path) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        log::error!("delete file failed, the error is {}", e.to_string());
                        Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
                    }
                }
            } else {
                match trash::delete(source_path) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        log::error!("delete file failed, the error is {}", e.to_string());
                        Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
                    }
                }
            }
        })?;
        Ok(())
    }

    pub fn rename_file(source: &str, target: &str) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let source_path = Path::new(source);
        let target_path = Path::new(target);
        if !source_path.exists() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "File not found")));
        }
        if target_path.exists() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "File already exists")));
        }
        fs::rename(source_path, target_path)?;
        Ok(())
    }
}