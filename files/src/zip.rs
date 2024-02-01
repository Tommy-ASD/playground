use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Seek, Write};
use std::iter::Iterator;
use tar::Builder;
use tokio::io::{AsyncRead, AsyncSeek, AsyncWrite};
use zip::result::ZipError;
use zip::write::FileOptions;

use std::fs::File;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &PathBuf,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            println!("adding file {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {path:?} as {name:?} ...");
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub async fn zip_folder_to_file(
    src_dir: &PathBuf,
    dst_file: &mut File,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    // let file = match std::fs::File::create(dst_file) {
    //     Ok(file) => file,
    //     Err(e) => return Err(ZipError::FileNotFound),
    // };

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, dst_file, method)?;

    Ok(())
}

pub async fn zip_folder_to_file_taking(
    src_dir: PathBuf,
    mut dst_file: File,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    zip_folder_to_file(&src_dir, &mut dst_file, method).await
}
