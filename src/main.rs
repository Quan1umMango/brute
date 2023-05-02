use anyhow::Result;
use clap::Parser;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

/// Bulk change names of files with index
#[derive(Parser)]
struct Cli {
    /// Path to the Directory
    path: std::path::PathBuf,
    ///Name to change replace with
    name: String,
    /// Starting index of number
    index: i32,
    /// Format of file. Example: txt, mp3, png, rs
    file_format: String,
}

impl Cli {
    fn check_path_validity(&self) -> Result<bool> {
        if self.path.as_path().is_dir() {
            return Ok(true);
        } else {
            panic!("Path is not a directory. Make sure the path is a directory and not a file and the directory actually exists.")
        }
    }

    fn get_all_files_with_format(
        &self,
        extension_param: String,
    ) -> Result<Vec<std::path::PathBuf>> {
        let mut out: Vec<std::path::PathBuf> = Vec::new();

        if self.check_path_validity().unwrap() {
            for file in fs::read_dir(self.path.as_path()).unwrap() {
                let file = file.unwrap();
                let path = file.path();
                let extension = path
                    .extension()
                    .and_then(OsStr::to_str)
                    .expect("No extension");
                if extension == extension_param {
                    out.push(path);
                }
            }
        }
        Ok(out)
    }

    fn rename_files(&self, files: Vec<PathBuf>, extension: String) -> Result<()> {
        let mut count = 1;
        for file in files {
            fs::rename(
                file,
                format!(
                    "{}/{}{}.{}",
                    self.path.to_str().unwrap(),
                    self.name,
                    count,
                    extension
                ),
            )?;
            count += 1;
        }
        Ok(())
    }

    fn start(&self) -> Result<()> {
        match self.get_all_files_with_format(self.file_format.to_string()) {
            Ok(files) => {
                self.rename_files(files, self.file_format.to_string())?;
            }
            Err(err) => return Err(err), // lol.
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    args.start()?;
    Ok(())
}
