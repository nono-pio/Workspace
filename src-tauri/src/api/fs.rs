use std::{
    fs,
    io::Error,
    path::{Path, PathBuf},
};

// region: ---Tauri Command

#[tauri::command]
pub async fn get_disk_entry_from_path(path: &str) -> Result<DiskEntry, String> {
    get_disk_entry(path).map_err(|err| err.to_string())
}

// endregion

// region: --- DiskEntry
#[derive(serde::Serialize)]
pub enum DiskEntry {
    File(File),
    Folder(Folder),
}

pub fn get_disk_entry<P: AsRef<Path>>(path: P) -> Result<DiskEntry, Error> {
    return get_disk_entry_path_buf(path.as_ref().to_path_buf());
}

pub fn get_disk_entry_path_buf(buf_path: PathBuf) -> Result<DiskEntry, Error> {
    if buf_path.is_dir() {
        return Ok(DiskEntry::Folder(Folder::generate(buf_path)?));
    } else {
        return Ok(DiskEntry::File(File::generate(buf_path)?));
    }
}

// endregion

// region: ---File
#[derive(serde::Serialize)]
pub struct File {
    name: Option<String>,      // dot file or file name
    extension: Option<String>, // .txt
    data: Option<Vec<u8>>,
}

impl File {
    fn generate(buf_path: PathBuf) -> Result<File, Error> {
        let file_name = buf_path
            .file_stem()
            .map(|name| name.to_str().unwrap().to_string());
        let extension = buf_path
            .extension()
            .map(|extension| extension.to_str().unwrap().to_string());

        Ok(File {
            name: file_name,
            extension: extension,
            data: None,
        })
    }
}

// endregion

// region: ---Folder
#[derive(serde::Serialize)]
pub struct Folder {
    name: Option<String>,
    disk_entries: Vec<DiskEntry>,
}

impl Folder {
    fn generate(path: PathBuf) -> Result<Folder, Error> {
        let name = path
            .file_name()
            .map(|name| name.to_str().unwrap().to_string());
        let mut entries: Vec<DiskEntry> = Vec::new();
        let entries_result = fs::read_dir(path)?;

        for entry_result in entries_result {
            if let Ok(entry) = entry_result {
                entries.push(get_disk_entry_path_buf(entry.path())?);
            }
        }

        Ok(Folder {
            name: name,
            disk_entries: entries,
        })
    }
}

// endregion
