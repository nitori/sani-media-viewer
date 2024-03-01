export interface ViewerOptions {
    sortBy: 'n' | 'm',
    sortReverse: boolean,
    showHidden: boolean,
    fullScreen: boolean,
    zoom: 'contain' | 'cover' | 'none',
}

export interface FileEntry {
    name: string,
    path: string,
    mtime: number,
    symlink: boolean,
}

export interface FolderEntry {
    name: string,
    path: string,
    symlink: boolean,
}

export interface FolderList {
    canonical_path: string,
    folders: FolderEntry[],
    files: FileEntry[],
    hash: FolderHash,
}


export interface FolderHash {
    hash: string
    duration: {
        secs: number
        nanos: number
    }
}

export interface State {
    options: ViewerOptions
    last_folder: string
    last_media: string
}
