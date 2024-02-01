use async_zip::{base::write::ZipFileWriter, error::ZipError, Compression, ZipEntryBuilder};
use std::iter::Iterator;
use tokio::{fs::File, io::AsyncReadExt};

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
    // let mut writez
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
