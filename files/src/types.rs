use std::{fs::Permissions, path::PathBuf, time::UNIX_EPOCH};

use chrono::NaiveDateTime;

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
pub struct FileDownloadMetadata {
    pub size: u64,
    pub created_at: Option<u64>,
    pub last_accessed: Option<u64>,
    pub permissions: Option<Permissions>,
}

#[derive(Debug)]
pub struct FileDownloadData {
    pub name: String,
    pub path: PathBuf,
    pub filetype: FileType,
    pub mime_type: String,
    pub meta: Option<FileDownloadMetadata>,
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

        match self.filetype {
            FileType::File => self.render_file(path),
            FileType::Directory => self.render(
                path,
                "directory-link",
                "",
                "directory",
                None, // Some("<button>Calculate</button>"),
            ),
            _ => self.render_file(path),
        }
    }
    fn render_file(&self, path: &str) -> String {
        self.render(path, "download-link", "download", "download", None)
    }

    fn render(
        &self,
        path: &str,
        class: &str,
        anchor_attrs: &str,
        endpoint: &str,
        override_file_size: Option<&str>,
    ) -> String {
        let md = if let Some(md) = &self.meta {
            md
        } else {
            &FileDownloadMetadata {
                size: 0,
                last_accessed: None,
                created_at: None,
                permissions: None,
            }
        };

        let rendered = format!(
            r#"
        <tr class="file-item">
            <td><a href="/{endpoint}/{path}" class="{class}" {anchor_attrs}>{name}</a></td>
            <td>{file_size}</td>
            <td>{created_time}</td>
            <td>{last_accessed_time}</td>
        </tr>"#,
            name = self.name,
            file_size = override_file_size.unwrap_or(&format_bytes(md.size)),
            created_time = NaiveDateTime::from_timestamp_millis(
                (md.created_at.unwrap_or(0) * 1000).try_into().unwrap()
            )
            .unwrap(),
            last_accessed_time = NaiveDateTime::from_timestamp_millis(
                (md.last_accessed.unwrap_or(0) * 1000).try_into().unwrap()
            )
            .unwrap()
        );
        rendered
    }

    pub async fn from_file(value: std::fs::DirEntry) -> Self {
        let md = tokio::fs::metadata(value.path())
            .await
            .and_then(|metadata| {
                Ok(FileDownloadMetadata {
                    size: metadata.len(),
                    created_at: metadata
                        .modified()
                        .map(|ok| {
                            ok.duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_secs()
                        })
                        .ok(),
                    last_accessed: metadata
                        .accessed()
                        .map(|ok| {
                            ok.duration_since(UNIX_EPOCH)
                                .expect("Time went backwards")
                                .as_secs()
                        })
                        .ok(),
                    permissions: Some(metadata.permissions()),
                })
            })
            .ok();
        let mime = match mime_guess::from_path(&value.path()).first_raw() {
            Some(mime) => mime.to_string(),
            None => "application/octet-stream".to_string(),
        };
        FileDownloadData {
            name: value.file_name().to_str().unwrap().to_string(),
            path: value.path().to_path_buf(),
            filetype: value.path().to_path_buf().into(),
            mime_type: mime,
            meta: md,
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
