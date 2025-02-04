use std::{
    env, fs,
    io::{self},
    path::Path,
};
mod emacs_like;
mod help;
mod rhai_settings;

fn main() -> io::Result<()> {
    let args = env::args().collect::<Vec<String>>();

    let mut content = String::new();
    let mut path = Path::new("New_File.txt").to_path_buf();
    if args.len() == 2 {
        content = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
            if e.kind() != io::ErrorKind::NotFound {
                eprintln!("Failed to create {e}");
                std::process::exit(1);
            } else {
                "".to_string()
            }
        });
        let path_ref: &Path = args[1].as_ref();
        path = path_ref.to_path_buf();
    }

    if path.exists() && path == Path::new("New_File.txt").to_path_buf() {
        for s in 1.. {
            let name = format!("New_File ({}).txt", s);
            let new_path = Path::new(&name);
            if new_path.exists() {
                println!("{}: exists", s);
                continue;
            } else {
                path = new_path.to_path_buf();
                break;
            }
        }

        let file_name = path.to_str().unwrap();
        emacs_like::lib::run(content, file_name, file_name)?;
    } else if !path.exists() {
        let file_name = format!("{} (new)", path.display());
        emacs_like::lib::run(content, &file_name, path.to_str().unwrap())?;
    } else {
        let file_name = format!("{}", path.display());
        emacs_like::lib::run(content, &file_name, path.to_str().unwrap())?;
    }

    Ok(())
}
