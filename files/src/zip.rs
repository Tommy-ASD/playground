use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Seek, Write};
use std::iter::Iterator;
use tar::Builder;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncWrite};
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

type SendableDirEntryIterator = dyn Iterator<Item = DirEntry> + Send;

async fn zip_dir<T>(
    it: &mut SendableDirEntryIterator,
    prefix: &PathBuf,
    writer: T,
    method: zip::CompressionMethod,
    progress_sender: Option<tokio::sync::mpsc::Sender<u64>>,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek + Send,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = zip::write::FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    let total_files = it.size_hint().0 as u64;
    let mut current_file_count = 0;

    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        dbg!();
        if path.is_file() {
            dbg!();
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options.clone())?;
            let mut f = tokio::fs::File::open(path).await?;

            f.read_to_end(&mut buffer).await?;
            zip.write_all(&buffer)?;
            buffer.clear();
            println!("added file {path:?} as {name:?} ...");
        } else if !name.as_os_str().is_empty() {
            dbg!();
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options.clone())?;
            println!("added dir {path:?} as {name:?} ...");
        }

        current_file_count += 1;
        if let Some(sender) = &progress_sender {
            println!("Sending {current_file_count} of {total_files}");
            // Send progress update
            dbg!();
            if sender.send(current_file_count).await.is_err() {
                dbg!();
                // Handle sender dropped
                return Err(zip::result::ZipError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Progress sender dropped",
                )));
            } else {
                dbg!();
            }
        }
        dbg!();
    }
    dbg!();

    zip.finish()?;
    dbg!();
    Result::Ok(())
}

pub async fn zip_folder_to_file(
    src_dir: &PathBuf,
    dst_file: &mut File,
    method: zip::CompressionMethod,
    progress_sender: Option<tokio::sync::mpsc::Sender<u64>>,
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
    progress_sender: Option<tokio::sync::mpsc::Sender<u64>>,
) -> zip::result::ZipResult<()> {
    zip_folder_to_file(&src_dir, &mut dst_file, method, progress_sender).await
}
