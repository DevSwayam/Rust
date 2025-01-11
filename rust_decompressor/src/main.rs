use std::{fs, io};

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
    }

    // * Dereferencing the value means accessing the value of args[1], which is a String
    // then borrowing the slice (&str) using &.
    let file_name = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&file_name).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let out_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let comment = file.comment();
        if !comment.is_empty() {
            println!("File {} comment: {} ", i, comment);
        }
        if file.name().ends_with('/') {
            println!("File {} extracted to \"{}\"", i, out_path.display());
            fs::create_dir_all(&out_path).unwrap();
        } else {
            println!(
                "File {} is extracted to \"{}\"({} bytes)",
                i,
                out_path.display(),
                file.size()
            );
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(&parent).unwrap();
                }
            }
            let mut out_file = fs::File::create(&out_path).unwrap();
            io::copy(&mut file, &mut out_file).unwrap();
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    return 0;
}
