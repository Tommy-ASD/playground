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

pub fn zip_folder_to_file(
    src_dir: &PathBuf,
    dst_file: &PathBuf,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let file = match std::fs::File::create(dst_file) {
        Ok(file) => file,
        Err(e) => return Err(ZipError::FileNotFound),
    };

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}

pub fn create_tar(src_dir: &str, dst_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tarball_file = File::create(dst_file)?;

    let mut tarball = Builder::new(tarball_file);

    tarball.append_dir_all("", src_dir)?;

    tarball.finish()?;

    Ok(())
}

pub fn create_gz(
    src_dir: &str,
    dst_file: &str,
    compression: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut tarball_file: Box<dyn Write> = Box::new(File::create(dst_file)?);

    tarball_file = Box::new(GzEncoder::new(tarball_file, Compression::new(compression)));

    let mut tarball = Builder::new(tarball_file);

    tarball.append_dir_all("", src_dir)?;

    tarball.finish()?;

    Ok(())
}
