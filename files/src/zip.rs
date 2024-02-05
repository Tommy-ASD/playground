use std::io::{Seek, Write};
use std::iter::Iterator;
use std::os::windows::fs::MetadataExt;
use tokio::io::AsyncReadExt;
use zip::result::ZipError;

use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

type SendableDirEntryIterator = dyn Iterator<Item = DirEntry> + Send;

async fn zip_dir<T>(
    it: &mut SendableDirEntryIterator,
    prefix: &PathBuf,
    writer: T,
    method: zip::CompressionMethod,
    progress_sender: Option<tokio::sync::mpsc::Sender<(u64, u64)>>,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek + Send,
{
    let it = it.collect::<Vec<DirEntry>>();
    let it_clone = it.clone();
    let mut zip = zip::ZipWriter::new(writer);
    let options = zip::write::FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let (total_files, total_size) = calculate_total_size(&mut it_clone.into_iter(), prefix).await?;
    if let Some(progress_sender) = &progress_sender {
        if progress_sender
            .send((total_files, total_size))
            .await
            .is_err()
        {
            // Handle sender dropped
            return Err(zip::result::ZipError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Progress sender dropped",
            )));
        };
    }

    let mut current_file_count = 0;
    let mut current_size = 0;

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options.clone())?;
            let mut f = tokio::fs::File::open(path).await?;

            f.read_to_end(&mut buffer).await?;
            zip.write_all(&buffer)?;
            buffer.clear();

            current_size += f.metadata().await?.len();
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options.clone())?;
        }

        current_file_count += 1;

        if let Some(sender) = &progress_sender {
            // Send progress update
            if sender
                .send((current_file_count, current_size))
                .await
                .is_err()
            {
                // Handle sender dropped
                return Err(zip::result::ZipError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Progress sender dropped",
                )));
            }
        }
    }

    zip.finish()?;
    Result::Ok(())
}

async fn calculate_total_size(
    it: &mut SendableDirEntryIterator,
    prefix: &PathBuf,
) -> std::io::Result<(u64, u64)> {
    let mut total_files = 0;
    let mut total_size = 0;

    for entry in it {
        let entry = entry;
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            total_files += 1;
            total_size += entry.metadata()?.file_size();
        } else if !name.as_os_str().is_empty() {
            total_files += 1;
        }
    }

    Ok((total_files, total_size))
}

pub async fn zip_folder_to_file(
    src_dir: &PathBuf,
    dst_file: &mut File,
    method: zip::CompressionMethod,
    progress_sender: Option<tokio::sync::mpsc::Sender<(u64, u64)>>,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    dbg!();
    zip_dir(
        &mut it.filter_map(|e| e.ok()),
        src_dir,
        dst_file,
        method,
        progress_sender,
    )
    .await?;
    dbg!();

    Ok(())
}

pub async fn zip_folder_to_file_taking(
    src_dir: PathBuf,
    mut dst_file: File,
    method: zip::CompressionMethod,
    progress_sender: Option<tokio::sync::mpsc::Sender<(u64, u64)>>,
) -> zip::result::ZipResult<()> {
    zip_folder_to_file(&src_dir, &mut dst_file, method, progress_sender).await
}
