use std::{fs::Metadata, path::PathBuf};

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
    pub mime_type: String,
}

impl FileDownloadData {
    pub async fn to_list_element(&self) -> String {
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

        let md: Result<Metadata, std::io::Error> = tokio::fs::metadata(binding.to_string()).await;
        match self.filetype {
            FileType::File => self.render_file(path, md),
            FileType::Directory => {
                format!(
                    r#"
<li class="file-item">
    <a href="/directory/{path}" class="directory-link" id={name}>{name}</a>
</li>"#,
                    name = self.name
                )
            }
            _ => self.render_file(path, md),
        }
    }
    fn render_file(&self, path: &str, md: Result<Metadata, std::io::Error>) -> String {
        match md {
            Ok(md) => {
                format!(
                    r#"
<li class="file-item">
    <a href="/download/{path}" class="download-link" download>{name}</a>
    <div class="file-metadata">
        <span class="file-size">size: {size}</span><br/>
        <span class="file-created">created: {created:?}</span><br/>
        <span class="file-last-accessed">accessed: {last_accessed:?}</span><br/>
        <span class="file-permissions">permissions: {permissions:?}</span><br/>
    </div>
</li>"#,
                    name = self.name,
                    size = format_bytes(md.len()),
                    created = md.created().unwrap(),
                    last_accessed = md.accessed().unwrap(),
                    permissions = md.permissions()
                )
            }
            Err(e) => {
                eprintln!("Metadata for file {path} failed: {e}");
                format!(
                    r#"
<li class="file-item">
    <a href="/download/{path}" class="download-link" download>{name}</a>
</li>"#,
                    name = self.name,
                )
            }
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let out = if bytes < (KB as u64) {
        format!("{} bytes", bytes)
    } else if bytes < (MB as u64) {
        format!("{:.3} KB", bytes as f64 / KB)
    } else if bytes < (GB as u64) {
        format!("{:.3} MB", bytes as f64 / MB)
    } else if bytes < (TB as u64) {
        format!("{:.3} GB", bytes as f64 / GB)
    } else {
        format!("{:.3} TB", bytes as f64 / TB)
    };

    out
}
