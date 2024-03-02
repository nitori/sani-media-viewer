// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, env, thread};
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::{Instant, Duration, UNIX_EPOCH};
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
    hash: FolderHash,
}

#[derive(Serialize)]
struct FolderHash {
    hash: String,
    duration: Duration,
}

const EXTENSIONS: [&'static str; 10] = [
    ".jpg",
    ".jpeg",
    ".jfif",
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

fn calculate_folder_hash(path: &PathBuf) -> Result<FolderHash, std::io::Error> {
    let start = Instant::now();
    let mut names: Vec<String> = vec![];

    let readdir = fs::read_dir(path)?;
    for entry in readdir {
        let Ok(direntry) = entry else {
            continue;
        };

        let filename = direntry.file_name().to_owned();
        let strname: String = filename.to_string_lossy().into();

        let Ok(meta) = direntry.metadata() else {
            warn!("Could not get metadata for {:?}", strname);
            continue;
        };

        let Ok(mtime) = meta.modified() else {
            warn!("Could not get modified time for {:?}", strname);
            continue;
        };

        let lowercase = strname.to_ascii_lowercase();
        if EXTENSIONS.iter().all(|v| !lowercase.ends_with(v)) {
            continue;
        }

        let mtime = match mtime.duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs_f64(),
            Err(_) => 0.0
        };

        names.push(strname.into());
        names.push(mtime.to_string());
    }

    names.sort();

    let mut hasher = Sha256::new();
    for name in &names {
        hasher.update(name.as_bytes());
    }

    let result = hasher.finalize().to_vec();
    let r2: Vec<_> = result.iter().map(|v| format!("{:02x}", v)).collect();

    Ok(FolderHash {
        hash: r2.join(""),
        duration: start.elapsed(),
    })
}

fn normalize_path(path: PathBuf) -> (String, PathBuf) {
    let canon_path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => path.clone(),
    };

    let mut normalized_path: String = path.to_string_lossy().into();
    normalized_path = normalized_path.replace("\\", "/");

    // i don't care anymore
    if (env::consts::OS == "windows") && normalized_path.starts_with("//?/") {
        normalized_path = normalized_path[4..].to_string();
    }

    (normalized_path, canon_path)
}

fn check_tilde(path: PathBuf) -> PathBuf {
    if path.starts_with("~\\") || path.starts_with("~/") {
        let Some(homedir) = home::home_dir() else {
            return path.clone();
        };

        let final_path = if path.starts_with("~\\") {
            let Ok(stripped_path) = path.strip_prefix("~\\") else {
                return path.clone();
            };
            stripped_path
        } else {
            let Ok(stripped_path) = path.strip_prefix("~/") else {
                return path.clone();
            };
            stripped_path
        };

        homedir.join(final_path)
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
            path: normalize_path(PathBuf::from(&drive)).0,
        });
    }

    let fav_folders_env = env::var("FAV_FOLDERS").unwrap_or("".into());
    info!("found configured fav folders:");

    let parts = fav_folders_env.split(";");
    for fav_folder in parts {
        info!(" - {}", fav_folder);

        let fav_path = check_tilde(PathBuf::from(fav_folder));
        info!(" - trying path: {:?}", fav_path.display());

        let Some(fav_name) = fav_path.file_name() else {
            continue;
        };

        if !fav_path.exists() {
            warn!("   `- Path doesn't exist!");
            continue;
        }

        favs.push(Favourite {
            name: fav_name.to_string_lossy().into(),
            path: normalize_path(fav_path).0,
        });
    }

    favs
}

#[tauri::command]
async fn get_list(path: String) -> Result<FolderList, String> {
    let input_path = if path.is_empty() {
        default_path()
    } else {
        PathBuf::from(&path)
    };

    let (normalized_path, canonical_path) = normalize_path(input_path.clone());

    // canonical_path is only used to test if the folder exists
    let Ok(current_meta) = canonical_path.metadata() else {
        return Err("Path does not exist".into());
    };
    if !canonical_path.exists() || !current_meta.is_dir() {
        return Err("Path does not exist".into());
    }

    let parent = match input_path.parent() {
        Some(path) => path.to_path_buf(),
        None => input_path.clone(),
    };

    let mut entries: Vec<PathBuf> = vec![];
    let paths = fs::read_dir(PathBuf::from(&normalized_path)).unwrap();
    for path in paths {
        if let Ok(direntry) = path {
            entries.push(direntry.path());
        } else {
            warn!("Could not read directory entry.");
        }
    }

    let mut files: Vec<FileEntry> = vec![];
    let mut folders: Vec<FolderEntry> = vec![FolderEntry {
        name: "..".into(),
        path: normalize_path(parent).0,
        symlink: false,
    }];

    if entries.len() > 50 {
        let chunk_size = entries.len() / (num_cpus::get() * 2 + 1);
        let chunks = entries.chunks(chunk_size);

        let mut receivers = vec![];
        for chunk in chunks {
            let (tx, rx) = mpsc::channel();
            let chunk_clone = chunk.to_vec().clone();

            thread::spawn(move || {
                let (fo, fi) = generate_partial_folder_list(chunk_clone);
                tx.send((fo, fi)).unwrap();
            });

            receivers.push(rx);
        }

        for handle in receivers {
            let (a, b) = handle.recv().unwrap();
            folders.extend(a);
            files.extend(b);
        }
    } else {
        let (a, b) = generate_partial_folder_list(entries);
        folders.extend(a);
        files.extend(b);
    }

    let Ok(hash) = calculate_folder_hash(&canonical_path) else {
        return Err("Could not calculate folder hash".into());
    };

    Ok(FolderList {
        canonical_path: normalized_path,
        folders,
        files,
        hash,
    })
}

#[tauri::command]
async fn get_folder_hash(path: String) -> Result<FolderHash, String> {
    if path.is_empty() {
        return Err("Path is empty".into());
    }

    let pathbuf = PathBuf::from(&path);

    let Ok(canon_path) = pathbuf.canonicalize() else {
        return Err("Error processing path".into());
    };

    if !canon_path.exists() {
        return Err("Path does not exist".into());
    }

    let Ok(meta) = canon_path.metadata() else {
        return Err("Error getting metadata".into());
    };

    if !meta.is_dir() {
        return Err("Path is not a directory".into());
    }

    let Ok(folder_hash) = calculate_folder_hash(&canon_path) else {
        return Err("Error calculating folder hash".into());
    };

    Ok(folder_hash)
}

fn generate_partial_folder_list(entries: Vec<PathBuf>) -> (Vec<FolderEntry>, Vec<FileEntry>) {
    let mut folders: Vec<FolderEntry> = vec![];
    let mut files: Vec<FileEntry> = vec![];

    for path in entries {
        let entry_name: String = match path.file_name() {
            Some(name) => name.to_string_lossy().into(),
            None => {
                warn!("Could not get file name.");
                continue;
            }
        };
        let Ok(canon_path) = path.canonicalize() else {
            warn!("Could not canonicalize path.");
            continue;
        };
        let Ok(meta_canon) = canon_path.metadata() else {
            warn!("Could not get direntry metadata of resolved path.");
            continue;
        };
        let Ok(meta) = path.metadata() else {
            warn!("Could not get direntry metadata.");
            continue;
        };

        if meta_canon.is_dir() {
            folders.push(FolderEntry {
                path: normalize_path(path).0,
                name: entry_name,
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

            let lowercase: String = entry_name.to_ascii_lowercase();
            if EXTENSIONS.iter().all(|v| !lowercase.ends_with(v)) {
                info!("Skipping non-image file: {}", entry_name);
                continue;
            }

            files.push(FileEntry {
                path: normalize_path(path).0,
                name: entry_name,
                mtime,
                symlink: meta.is_symlink(),
            });
        }
    }

    (folders, files)
}

#[tauri::command]
async fn get_file(path: String) -> Vec<u8> {
    let file = fs::read(path).unwrap();
    file
}


fn main() {
    dotenv().ok();
    env_logger::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_favourites,
            get_list,
            get_file,
            get_folder_hash,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
