use async_zip::error::ZipError;
use async_zip::tokio::write::ZipFileWriter;
use async_zip::ZipEntryBuilder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Seek, Write};
use std::iter::Iterator;
use tar::Builder;
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncReadExt, AsyncSeek, AsyncWrite},
};

use anyhow::anyhow;
use anyhow::bail;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub type Result<V> = std::result::Result<V, ZipError>;

type SendableDirEntryIterator = dyn Iterator<Item = DirEntry> + Send;

async fn zip_dir_async(
    it: &mut SendableDirEntryIterator,
    prefix: &PathBuf,
    writer: &mut File,
    method: async_zip::Compression,
) {
    let mut zip = async_zip::tokio::write::ZipFileWriter::with_tokio(writer);

    let data = b"This is an example file.";
    let builder = ZipEntryBuilder::new("bar.txt".into(), method);

    // builder.

    // let mut buffer = Vec::new();
    for entry in it {
        let entry_path = entry.path();
        let entry_str = entry_path
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("Directory file path not valid UTF-8."))
            .unwrap();

        write_entry(entry_str, entry_path, writer, method)
            .await
            .unwrap();
    }
    // zip.finish()?;
    // Result::Ok(())
}

async fn write_entry(
    filename: &str,
    input_path: &Path,
    writer: &mut ZipFileWriter<File>,
    method: async_zip::Compression,
) -> Result<()> {
    let mut input_file = File::open(input_path).await.unwrap();
    let input_file_size = input_file.metadata().await.unwrap().len() as usize;

    let mut buffer = Vec::with_capacity(input_file_size);
    input_file.read_to_end(&mut buffer).await.unwrap();

    let builder = ZipEntryBuilder::new(filename.into(), method);
    writer.write_entry_whole(builder, &buffer).await.unwrap();

    Ok(())
}

pub async fn zip_folder_to_file(
    src_dir: &PathBuf,
    dst_file: &mut File,
    method: async_zip::Compression,
) {
    if !Path::new(src_dir).is_dir() {
        panic!("AAAAAAAAAAAHHHH")
    }

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
