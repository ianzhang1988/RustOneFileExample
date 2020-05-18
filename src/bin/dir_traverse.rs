use std::fs::{read_dir, DirEntry};
use std::io;
use std::string::String;
use std::path::{Path, PathBuf};
use std::path::Prefix::Verbatim;

fn list_dir<P:AsRef<Path>>( path: P) -> io::Result<()> {
    for entry in read_dir(path)? {
        let entry = entry?;
        println!("{}", entry.file_name().into_string().or(
            Err(io::Error::new::<String>(io::ErrorKind::Other, "error".to_string())))?);
    }
    Ok(())
}

fn walk_dir_recursive<P:AsRef<Path>>( path: P, entrys: &mut Vec<DirEntry>) -> io::Result<()> {
    // would be nice if we have yield
    for entry in read_dir(path)? {
        let entry = entry?;

        let mut path: Option<PathBuf> = None;

        if entry.file_type()?.is_dir() {
            path = Some(entry.path());
        }

        entrys.push(entry);

        if let Some(p) = path {
            walk_dir_recursive(p, entrys);
        }
    }

    Ok(())
}

fn list_all_files<P:AsRef<Path>>( path: P) -> io::Result<()> {
    let mut dirs = Vec::<DirEntry>::new();
    walk_dir_recursive(path, &mut dirs)?;

    for e in &dirs {
        println!("{:?}", e);
    }

    Ok(())
}

fn main() {
    list_dir(".").expect("main error 1");

    list_all_files(".").expect("main error 2");
}