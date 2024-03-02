<template>
  <div id="main">
    <div id="left">
      <div id="shortcuts-meta">
        <Options :options="viewerOptions"
                 @option:sort-by="viewerOptions.sortBy = $event"
                 @option:sort-reverse="viewerOptions.sortReverse = $event"
                 @option:show-hidden="viewerOptions.showHidden = $event"
                 @option:full-screen="viewerOptions.fullScreen = $event"
                 @option:zoom="viewerOptions.zoom = $event"
        />
        <div id="meta">
          {{ currentMediaIndex + 1 }}/{{ currentFiles.length }}
        </div>
      </div>
      <div id="favs-folders">
        <Favourites @select:folder="currentFolder = $event"/>
        <Folders :previous="previousFolder"
                 :current="currentFolder"
                 :folders="currentFolders"
                 @select:folder="currentFolder = $event"/>
      </div>
      <Files :current="currentMedia"
             :files="currentFiles"
             @select:media="currentMedia = $event"/>
    </div>
    <div id="right">
      <div id="image" ref="imageContainer" @wheel="onMediaScroll" :class="{
        'object-fit-cover': viewerOptions.zoom === 'cover',
        'object-fit-none': viewerOptions.zoom === 'none',
      }" @fullscreenchange="onFullscreenChange">
        <MediaItem :media="currentMedia"/>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, onMounted, watch, computed} from "vue";
import Favourites from "./components/Favourites.vue";
import Folders from "./components/Folders.vue";
import Files from "./components/Files.vue";
import MediaItem from "./components/MediaItem.vue";
import Options from "./components/Options.vue";
import type {FolderHash, FolderList, ViewerOptions, FolderEntry, FileEntry, State} from "./types";
import {sortByName, sortByMtime, defaultOptions, defaultState} from "./utils.ts";
import {configDir, join} from '@tauri-apps/api/path';
import {readTextFile, writeTextFile, createDir} from '@tauri-apps/api/fs';
import {invoke} from "@tauri-apps/api/tauri";

const folderListing = ref<FolderList>({
  canonical_path: "",
  folders: [],
  files: [],
  hash: {
    hash: "",
    duration: {secs: 0, nanos: 0},
  },
});

const currentFolder = ref<FolderEntry | null>(null);
const previousFolder = ref<FolderEntry | null>(null);
const currentMedia = ref<FileEntry | null>(null);
const imageContainer = ref<HTMLDivElement | null>(null);
const viewerOptions = ref<ViewerOptions>(defaultOptions());

const currentFiles = ref<FileEntry[]>([]);
const currentFolders = ref<FolderEntry[]>([]);

const currentMediaIndex = computed(() => {
  let index = currentFiles.value.findIndex(f => f.path === currentMedia.value?.path);
  if (index === -1) {
    return 0;
  }
  return index;
});


function buildFiles() {
  let files = [...folderListing.value.files].filter(f => {
    return viewerOptions.value.showHidden || !f.name.startsWith('.');
  }).sort(viewerOptions.value.sortBy === 'm' ? sortByMtime : sortByName);
  if (viewerOptions.value.sortReverse) {
    files = files.reverse();
  }
  return files;
}

function buildFolders() {
  return [...folderListing.value.folders].filter(f => {
    return viewerOptions.value.showHidden || !f.name.startsWith('.') || f.name === '..';
  }).sort(sortByName);
}

function updateFilesAndFolders() {
  let lastMediaPath = currentMedia.value?.path;
  currentFiles.value = buildFiles();
  currentFolders.value = buildFolders();
  if (lastMediaPath) {
    let foundMedia = currentFiles.value.find(f => f.path === lastMediaPath) || null;
    if (!foundMedia) {
      foundMedia = currentFiles.value[0] || null;
    }
    currentMedia.value = foundMedia;
  }
}


watch(viewerOptions, async () => {
  updateFilesAndFolders();
  const state = await loadState();
  state.options = JSON.parse(JSON.stringify(viewerOptions.value)); // clone
  await saveState(state);
}, {deep: true});


watch(currentFolder, (newFolder, oldFolder) => {
  if (oldFolder) {
    previousFolder.value = oldFolder;
  }

  if (newFolder) {
    (async () => {
      const state = await loadState();
      state.last_folder = newFolder.path;
      await saveState(state);
      folderListing.value = await invoke("get_list", {path: newFolder.path});
      updateFilesAndFolders();
    })();
  }
});

watch(currentMedia, async (newMedia) => {
  let state = await loadState();
  if (newMedia) {
    state.last_media = newMedia.path;
  } else {
    state.last_media = '';
  }
  await saveState(state);
});

const getConfigPaths = async () => {
  const configDirPath = await configDir();
  const saniConfigFolder = await join(configDirPath, 'sani-media-viewer');
  await createDir(saniConfigFolder, {recursive: true});
  return {configDirPath, saniConfigFolder};
};

const saveState = async (state: State) => {
  const {saniConfigFolder} = await getConfigPaths();
  const stateText = JSON.stringify(state, undefined, 2);
  await writeTextFile(await join(saniConfigFolder, 'state.json'), stateText);
};

const loadState = async (): Promise<State> => {
  const {saniConfigFolder} = await getConfigPaths();
  let stateText: string = '';
  try {
    stateText = await readTextFile(await join(saniConfigFolder, 'state.json'));
    return JSON.parse(stateText);
  } catch (e) {
    const state = defaultState();
    await saveState(state);
    return state;
  }
};

onMounted(async () => {
  const state = await loadState();
  viewerOptions.value = state.options;
  folderListing.value = await invoke("get_list", {path: state.last_folder});
  if (state.last_media !== '') {
    currentMedia.value = folderListing.value.files.find(f => f.path === state.last_media) || null;
  }
  currentFolder.value = {
    path: folderListing.value.canonical_path,
    name: folderListing.value.canonical_path.split('/').pop() || '',
    symlink: false,
  };
  updateFilesAndFolders();

  check_folder_hash();
});


async function check_folder_hash() {
  try {
    const hash: FolderHash = await invoke("get_folder_hash", {path: folderListing.value.canonical_path});
    if (hash.hash !== folderListing.value.hash.hash) {
      folderListing.value = await invoke("get_list", {path: folderListing.value.canonical_path});
      updateFilesAndFolders();
    }
  } finally {
    window.setTimeout(check_folder_hash, 10000);
  }
}


const setToFirstMedia = () => {
  if (currentFiles.value.length > 0) {
    currentMedia.value = currentFiles.value[0];
  } else {
    currentMedia.value = null;
  }
};

const setToLastMedia = () => {
  if (currentFiles.value.length > 0) {
    currentMedia.value = currentFiles.value[currentFiles.value.length - 1];
  } else {
    currentMedia.value = null;
  }
};

const nextMedia = () => {
  if (currentFiles.value.length === 0) {
    return;
  }
  let index = Math.min(currentFiles.value.length - 1, currentMediaIndex.value + 1);
  currentMedia.value = currentFiles.value[index];
};

const prevMedia = () => {
  if (currentFiles.value.length === 0) {
    return;
  }
  let index = Math.max(0, currentMediaIndex.value - 1);
  currentMedia.value = currentFiles.value[index];
};


document.addEventListener('keydown', ev => {
  if (ev.key === 'PageDown') {
    ev.preventDefault();
    nextMedia();
  } else if (ev.key === 'PageUp') {
    ev.preventDefault();
    prevMedia();
  } else if (ev.key === 'Home') {
    ev.preventDefault();
    if (viewerOptions.value.zoom === 'contain') {
      setToFirstMedia();
      return;
    }

    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.setObjectPosition(0, 0);
    // }
  } else if (ev.key === 'End') {
    ev.preventDefault();
    if (viewerOptions.value.zoom === 'contain') {
      setToLastMedia();
      return;
    }
    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.setObjectPosition(100, 100);
    // }
  } else if (ev.key === 'f') {
    ev.preventDefault();
    viewerOptions.value.fullScreen = !viewerOptions.value.fullScreen;
  } else if (ev.key === 'h') {
    ev.preventDefault();
    viewerOptions.value.showHidden = !viewerOptions.value.showHidden;
  } else if (ev.key === 'r') {
    ev.preventDefault();
    viewerOptions.value.sortReverse = !viewerOptions.value.sortReverse;
  } else if (ev.key === 'n') {
    ev.preventDefault();
    viewerOptions.value.sortBy = 'n';
  } else if (ev.key === 'm') {
    ev.preventDefault();
    viewerOptions.value.sortBy = 'm';
  } else if (ev.key === 'z') {
    ev.preventDefault();
    if (viewerOptions.value.zoom === 'contain') {
      viewerOptions.value.zoom = 'cover'
    } else if (viewerOptions.value.zoom === 'cover') {
      viewerOptions.value.zoom = 'none'
    } else {
      viewerOptions.value.zoom = 'contain';
    }
  }
});


const onMediaScroll = (ev: WheelEvent) => {
  if (ev.deltaY < 0) {
    if (viewerOptions.value.zoom === 'contain') {
      prevMedia();
      return;
    }
    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.moveMediaUp();
    // }
  } else if (ev.deltaY > 0) {
    if (viewerOptions.value.zoom === 'contain') {
      nextMedia();
      return;
    }
    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.moveMediaDown();
    // }
  }
};


// Fullscreen stuff
const watchedFullscreen = computed(() => {
  return viewerOptions.value.fullScreen;
});

watch(watchedFullscreen, (doFullscreen) => {
  if (imageContainer.value === null) {
    viewerOptions.value.fullScreen = false;
    return;
  }
  if (doFullscreen && !document.fullscreenElement) {
    if (imageContainer.value.requestFullscreen) {
      imageContainer.value.requestFullscreen().then(result => {
        if (result === undefined) { // yes, undefined
          document.documentElement.classList.add('fullscreen');
        }
      });
    }
  } else if (!doFullscreen && document.fullscreenElement) {
    document.documentElement.classList.remove('fullscreen');
    document.exitFullscreen();
  } else {
    // should not appear, but just in case
    viewerOptions.value.fullScreen = false;
    document.documentElement.classList.remove('fullscreen');
    if (document.fullscreenElement) {
      document.exitFullscreen();
    }
  }
});

const onFullscreenChange = (_ev: Event) => {
  if (document.fullscreenElement === null && viewerOptions.value.fullScreen) {
    viewerOptions.value.fullScreen = false;
  }
};

</script>