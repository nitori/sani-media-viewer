// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, env};
use std::path::PathBuf;
use std::time::{UNIX_EPOCH};
use home;
use dotenv::dotenv;
use log::{info, warn};
use sha2::{Sha256, Digest};
use serde::{Serialize};

#[derive(Serialize)]
struct Favourite {
    name: String,
    path: String,
}

#[derive(Serialize)]
struct FolderEntry {
    name: String,
    path: String,
    symlink: bool,
}

#[derive(Serialize)]
struct FileEntry {
    name: String,
    path: String,
    mtime: f64,
    symlink: bool,
}

#[derive(Serialize)]
struct FolderList {
    canonical_path: String,
    folders: Vec<FolderEntry>,
    files: Vec<FileEntry>,
    hash: String,
}

#[derive(Serialize)]
struct FolderHash {
    hash: String,
}

const EXTENSIONS: [&'static str; 9] = [
    ".jpg",
    ".jpeg",
    ".png",
    ".svg",
    ".gif",
    ".webp",
    ".webm",
    ".mp4",
    ".mkv",
];

fn listdrives() -> Vec<String> {
    let mut drives = vec![];

    if env::consts::OS == "windows" {
        for drive in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
            let drivestr = format!("{}:\\", drive);
            let path = PathBuf::from(drivestr.clone());
            if path.exists() {
                drives.push(drivestr)
            }
        }
    }

    drives
}

fn default_path() -> PathBuf {
    let home_path = home::home_dir().unwrap();
    let pictures = home_path.join("Pictures");
    if pictures.exists() {
        pictures
    } else {
        home_path
    }
}

fn resolve_path(path: &String) -> PathBuf {
    if path.is_empty() {
        default_path()
    } else {
        PathBuf::from(path).canonicalize().unwrap_or_else(|_| default_path())
    }
}

fn calculate_folder_hash(path: PathBuf) -> Result<String, std::io::Error> {
    let mut names: Vec<String> = vec![];

    let readdir = fs::read_dir(path)?;
    for entry in readdir {
        let Ok(direntry) = entry else {
            continue;
        };

        let filename = direntry.file_name().to_owned();
        let Some(strname) = filename.to_str() else {
            continue;
        };
        names.push(strname.into());
    }

    names.sort();

    let mut hasher = Sha256::new();
    for name in &names {
        hasher.update(name.as_bytes());
    }

    let result = hasher.finalize().to_vec();
    let r2: Vec<_> = result.iter().map(|v| format!("{:02x}", v)).collect();

    Ok(r2.join(""))
}

fn normalize_path(path: PathBuf) -> String {
    let Ok(fixed_path) = dunce::canonicalize(path.clone()) else {
        return path.to_string_lossy().into();
    };
    let mut normalized_path: String = fixed_path.to_string_lossy().into();
    normalized_path = normalized_path.replace("\\", "/");
    normalized_path
}

fn check_tilde(path: PathBuf) -> PathBuf {
    if path.starts_with("~\\") {
        let Some(homedir) = home::home_dir() else {
            return path.clone();
        };

        let Ok(stripped_path) = path.strip_prefix("~\\") else {
            return path.clone();
        };

        homedir.join(stripped_path)
    } else {
        path.clone()
    }
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_favourites() -> Vec<Favourite> {
    let mut favs: Vec<Favourite> = vec![];

    let drives = listdrives();
    for drive in drives {
        favs.push(Favourite {
            name: drive.clone(),
            path: normalize_path(PathBuf::from(&drive)),
        });
    }

    let fav_folders_env = env::var("FAV_FOLDERS").unwrap_or("".into());
    info!("found configured fav folders:");

    let parts = fav_folders_env.split(";");
    for fav_folder in parts {
        info!(" - {}", fav_folder);

        let fav_path = check_tilde(PathBuf::from(fav_folder));

        let Some(fav_name) = fav_path.file_name() else {
            continue;
        };

        if !fav_path.exists() {
            warn!("   `- Path doesn't exist!");
            continue;
        }

        favs.push(Favourite {
            name: fav_name.to_string_lossy().into(),
            path: normalize_path(fav_path),
        });
    }

    favs
}

#[tauri::command]
async fn get_list(path: String) -> FolderList {
    let canonical_path = resolve_path(&path);
    let hash = calculate_folder_hash(canonical_path.clone()).unwrap_or("".into());

    let parent = match canonical_path.parent() {
        Some(path) => path.to_path_buf(),
        None => canonical_path.clone(),
    };

    let mut folders: Vec<FolderEntry> = vec![FolderEntry {
        name: "..".into(),
        path: normalize_path(parent),
        symlink: false,
    }];

    let mut files: Vec<FileEntry> = vec![];

    let paths = fs::read_dir(canonical_path.clone()).unwrap();
    for path in paths {
        let Ok(direntry) = path else {
            warn!("Could not unwrap path");
            continue;
        };
        let Ok(meta) = direntry.metadata() else {
            warn!("Could not get direntry metadata.");
            continue;
        };

        if meta.is_dir() {
            folders.push(FolderEntry {
                path: normalize_path(direntry.path()),
                name: direntry.file_name().to_string_lossy().into(),
                symlink: meta.is_symlink(),
            })
        } else {
            let mtime = match meta.modified() {
                Ok(time) => match time.duration_since(UNIX_EPOCH) {
                    Ok(duration) => duration.as_secs_f64(),
                    Err(_) => 0.0
                },
                Err(_) => 0.0
            };

            let lowercase: String = direntry.file_name().to_ascii_lowercase().to_string_lossy().into();
            if EXTENSIONS.iter().all(|v| !lowercase.ends_with(v)) {
                continue;
            }

            files.push(FileEntry {
                path: normalize_path(direntry.path()),
                name: direntry.file_name().to_string_lossy().into(),
                mtime,
                symlink: meta.is_symlink(),
            });
        }
    }

    FolderList {
        canonical_path: normalize_path(canonical_path),
        folders,
        files,
        hash,
    }
}

#[tauri::command]
async fn get_file(path: String) -> Vec<u8> {
    let file = fs::read(path).unwrap();
    file
}


fn main() {
    dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_favourites,
            get_list,
            get_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
