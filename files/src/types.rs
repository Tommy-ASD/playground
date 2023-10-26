use std::path::PathBuf;

#[derive(Debug)]
pub enum FileType {
    Directory,
    File,
    Symlink,
    Unknown,
}

impl From<PathBuf> for FileType {
    fn from(value: PathBuf) -> Self {
        if value.is_dir() {
            Self::Directory
        } else if value.is_file() {
            Self::File
        } else if value.is_symlink() {
            Self::Symlink
        } else {
            Self::Unknown
        }
    }
}

#[derive(Debug)]
pub struct FileDownloadData {
    pub name: String,
    pub path: PathBuf,
    pub filetype: FileType,
}

impl FileDownloadData {
    pub fn to_list_element(&self) -> String {
        let binding = self.path.to_string_lossy();
        let mut path = binding
            .strip_prefix(dotenv!("STORAGE_PATH"))
            .unwrap_or(&self.name);
        if let Some(inner_path) = path.strip_prefix("\\") {
            path = inner_path;
        }
        if let Some(inner_path) = path.strip_prefix("/") {
            path = inner_path;
        }

        match self.filetype {
            FileType::File => {
                format!(
                    r#"
<li class="file-item">
    <a href="/downloads/{path}" class="download-link" download>{name}</a>
</li>"#,
                    name = self.name
                )
            }
            FileType::Directory => {
                format!(
                    r#"
<li class="file-item">
    <a href="/directory/{path}" class="directory-link" id={name}>{name}</a>
</li>"#,
                    name = self.name
                )
            }
            _ => {
                format!(
                    r#"
<li class="file-item">
    <a href="/downloads/{path}" class="download-link" download>{name}</a>
</li>"#,
                    name = self.name
                )
            }
        }
    }
}
