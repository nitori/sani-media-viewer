import type {FileEntry, FolderEntry, State, ViewerOptions} from "./types";

export const defaultOptions = (): ViewerOptions => {
    return {
        sortBy: "n",
        sortReverse: false,
        showHidden: false,
        fullScreen: false,
        zoom: 'contain',
    };
};

export const defaultState = (): State => {
    return {
        options: defaultOptions(),
        last_folder: '',
        last_media: '',
    };
};

export const posWithin = (itemEl: HTMLElement, parentEl: HTMLElement) => {
    let rect = itemEl.getBoundingClientRect();
    let parentRect = parentEl.getBoundingClientRect();
    return rect.top >= parentRect.top && rect.bottom <= parentRect.bottom;
}

export const focusElementInParent = (itemEl: HTMLElement, parentEl: HTMLElement) => {
    if (itemEl === null || parentEl === null) {
        return;
    }
    if (posWithin(itemEl, parentEl)) {
        return;
    }
    itemEl.scrollIntoView({behavior: 'instant', block: 'center'});
}

export const sortByMtime = (a: FileEntry, b: FileEntry) => {
    return a.mtime - b.mtime;
};

export const sortByName = (a: FileEntry | FolderEntry, b: FileEntry | FolderEntry) => {
    let aVal = a.name;
    let bVal = b.name;
    aVal = aVal.toLowerCase().replace(/[\[\](){}<>.]+/g, '');
    bVal = bVal.toLowerCase().replace(/[\[\](){}<>.]+/g, '');
    if (aVal === '..') return -1;
    if (bVal === '..') return 1;
    if (aVal === bVal) return 0;
    if (aVal < bVal) return -1;
    return 1;
};

export const isVideo = (name: string) => {
    return name.match(/\.(mp4|webm|mkv)$/);
};