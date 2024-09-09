use std::{
    fs::{create_dir_all, remove_file, symlink_metadata},
    io::{stdin, stdout, ErrorKind, Write},
    os::unix::fs::symlink,
    path::{Path, PathBuf},
    process,
};

use clap::{Parser, Subcommand};

const REBOS_FILES_PATH: &str = "/home/julius/.config/rebos/files/";

#[derive(Parser, Debug)]
#[command(name = "files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Add { path: PathBuf },
    #[command(arg_required_else_help = true)]
    Remove { path: PathBuf },
    #[command(arg_required_else_help = true)]
    Trim {
        path: PathBuf,
        #[arg(default_value_t = false, short, long)]
        end: bool,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add { path } => add(&path),
        Commands::Remove { path } => remove(&path),
        Commands::Trim { end, path } => {
            let path = trim(&path);
            println!(
                "{}",
                match end {
                    true => path
                        .parent()
                        .expect("Path shouldnt be just root or empty")
                        .display(),
                    false => path.display(),
                }
            );
        }
    }
}

fn add(path: &Path) {
    let origin = PathBuf::from(REBOS_FILES_PATH).join(path);
    let link = trim(path);

    // Check if the path already exists
    while let Ok(metadata) = symlink_metadata(&link) {
        // Check if it is a symlink
        if metadata.is_symlink() {
            return;
        }
        // Ask for retry, if not, abort
        if bool_question(&format!(
            "The path {} already exists and isn't a symlink, retry?",
            link.display()
        )) {
            continue;
        }
        process::exit(1)
    }
    println!("Symlinking {}", link.display());
    create_symlink(&origin, &link);
}

#[expect(clippy::wildcard_enum_match_arm)]
fn create_symlink(origin: &Path, link: &Path) {
    if let Err(e) = symlink(origin, link) {
        match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Insufficient permissions to create the symlink");
                process::exit(1)
            }
            ErrorKind::NotFound => {
                if bool_question(&format!(
                    "Could not find the path {}, should the parent paths be created?",
                    link.display()
                )) {
                    if let Err(e) =
                        create_dir_all(link.parent().expect("Path shouldnt be just root or empty"))
                    {
                        match e.kind() {
                            ErrorKind::PermissionDenied => {
                                println!("Insufficient permissions to create parent directories");
                                process::exit(1)
                            }
                            other => println!("Error creating parent directory: {other:?}"),
                        }
                    } else {
                        create_symlink(origin, link);
                    }
                } else {
                    process::exit(1)
                }
            }
            other => {
                println!("Error creating symlink: {other:?}");
            }
        }
    };
}

#[expect(clippy::wildcard_enum_match_arm)]
fn remove(path: &Path) {
    let path = trim(path);
    if let Err(e) = remove_file(&path) {
        match e.kind() {
            ErrorKind::PermissionDenied => {
                println!("Insufficient permissions to delete symlink");
                process::exit(1)
            }
            other => println!("Error deleting symlink: {other:?}"),
        }
    }
}

fn trim(path: &Path) -> PathBuf {
    let root = PathBuf::from("/");
    let path: PathBuf = path.components().skip(1).collect();

    root.join(path)
}

#[expect(clippy::let_underscore_must_use)]
fn bool_question(question: &str) -> bool {
    print!("{question} ");
    let _ = stdout().flush();

    let mut buffer = String::new();

    loop {
        buffer.clear();
        let _ = stdin().read_line(&mut buffer);

        match buffer.trim() {
            "y" | "Y" | "yes" | "Yes" => return true,
            "n" | "N" | "no" | "No" => return false,
            _other => continue,
        }
    }
}
