use async_zip::error::ZipError;
use async_zip::ZipEntryBuilder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Seek, Write};
use std::iter::Iterator;
use tar::Builder;
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};

use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub type Result<V> = std::result::Result<V, ZipError>;

type SendableDirEntryIterator = dyn Iterator<Item = DirEntry> + Send;

async fn zip_dir_async(
    it: &mut SendableDirEntryIterator,
    prefix: &PathBuf,
    writer: &mut tokio::fs::File,
    method: async_zip::Compression,
) {
    let mut zip = async_zip::tokio::write::ZipFileWriter::with_tokio(writer);

    let data = b"This is an example file.";
    let builder = ZipEntryBuilder::new("bar.txt".into(), async_zip::Compression::Deflate);

    // builder.

    // let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            // println!("adding file {path:?} as {name:?} ...");
            // #[allow(deprecated)]
            // zip.start_file_from_path(name, options)?;
            // let mut f = File::open(path)?;

            // f.read_to_end(&mut buffer)?;
            // zip.write_all(&buffer)?;
            // buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // // Only if not root! Avoids path spec / warning
            // // and mapname conversion failed error on unzip
            // println!("adding dir {path:?} as {name:?} ...");
            // #[allow(deprecated)]
            // zip.add_directory_from_path(name, options)?;
        }
    }
    // zip.finish()?;
    // Result::Ok(())
}

pub async fn zip_folder_to_file(
    src_dir: &PathBuf,
    dst_file: &mut tokio::fs::File,
    method: async_zip::Compression,
) {
    if !Path::new(src_dir).is_dir() {
        panic!("AAAAAAAAAAAHHHH")
    }

    // let file = match std::fs::File::create(dst_file) {
    //     Ok(file) => file,
    //     Err(e) => return Err(ZipError::FileNotFound),
    // };

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir_async(&mut it.filter_map(|e| e.ok()), src_dir, dst_file, method).await;
}

async fn walk_dir(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let mut dirs = vec![dir];
    let mut files = vec![];

    while dirs.is_empty() {
        let mut dir_iter = tokio::fs::read_dir(dirs.remove(0)).await?;

        while let Some(entry) = dir_iter.next_entry().await? {
            let entry_path_buf = entry.path();

            if entry_path_buf.is_dir() {
                dirs.push(entry_path_buf);
            } else {
                files.push(entry_path_buf);
            }
        }
    }

    Ok(files)
}
