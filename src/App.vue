<template>
  <div id="main">
    <div id="left">
      <div id="shortcuts-meta">
        <Options v-model="viewerOptions"/>
        <div id="meta">
          {{ calculatedIndex + 1 }}/{{ currentFiles.length }}
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
import type {FolderList, ViewerOptions, FolderEntry, FileEntry, State} from "./types";
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
const currentMediaIndex = ref<number>(-1);
const imageContainer = ref<HTMLDivElement | null>(null);

const viewerOptions = ref<ViewerOptions>(defaultOptions());

let skipRerendering = false;

watch(viewerOptions, async () => {
  const state = await loadState();
  state.options = JSON.parse(JSON.stringify(viewerOptions.value)); // clone
  await saveState(state);
}, {deep: true});

watch(currentFolder, (newFolder, oldFolder) => {
  if (oldFolder) {
    previousFolder.value = oldFolder;
  }

  if (newFolder && !skipRerendering) {
    (async () => {
      const state = await loadState();
      state.canonical_path = newFolder.path;
      await saveState(state);
      folderListing.value = await invoke("get_list", {path: newFolder.path});
      setIndex(calculateIndex(0));
    })();
  }
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
  folderListing.value = await invoke("get_list", {path: state.canonical_path});
  skipRerendering = true;
  currentFolder.value = {
    path: folderListing.value.canonical_path,
    name: folderListing.value.canonical_path.split('/').pop() || '',
    symlink: false,
  };
  setIndex(calculateIndex(0));
  window.setTimeout(() => {
    skipRerendering = false;
  }, 500);
});

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

const currentFiles = computed(() => {
  let files = [...folderListing.value.files].filter(f => {
    return viewerOptions.value.showHidden || !f.name.startsWith('.');
  }).sort(viewerOptions.value.sortBy === 'm' ? sortByMtime : sortByName);
  if (viewerOptions.value.sortReverse) {
    files = files.reverse();
  }
  return files;
});

const currentFolders = computed(() => {
  return [...folderListing.value.folders].filter(f => {
    return viewerOptions.value.showHidden || !f.name.startsWith('.') || f.name === '..';
  }).sort(sortByName);
});

const calculatedIndex = computed(() => {
  return calculateIndex(currentMediaIndex.value);
});

const calculateIndex = (index: number) => {
  if (viewerOptions.value.sortReverse) {
    return currentFiles.value.length - 1 - index;
  }
  return index;
};

const onFullscreenChange = (_ev: Event) => {
  if (document.fullscreenElement === null && viewerOptions.value.fullScreen) {
    viewerOptions.value.fullScreen = false;
  }
};

const setIndex = (index: number) => {
  index = Math.min(currentFiles.value.length - 1, Math.max(0, index));
  currentMediaIndex.value = index;
  if (index < 0) {
    currentMedia.value = null;
  } else {
    if (viewerOptions.value.sortReverse) {
      index = currentFiles.value.length - 1 - index;
    }
    currentMedia.value = currentFiles.value[index];
  }
};

const nextIndex = () => {
  if (viewerOptions.value.sortReverse) {
    setIndex(currentMediaIndex.value - 1);
  } else {
    setIndex(currentMediaIndex.value + 1);
  }
};

const prevIndex = () => {
  if (viewerOptions.value.sortReverse) {
    setIndex(currentMediaIndex.value + 1);
  } else {
    setIndex(currentMediaIndex.value - 1);
  }
};


watch(currentMedia, (value) => {
  // find in list
  if (value === null) {
    setIndex(-1);
    return;
  }
  let index = currentFiles.value.findIndex(f => f.path === value.path);
  if (index === -1) {
    setIndex(-1);
    return;
  }
  index = calculateIndex(index);
  if (index !== calculatedIndex.value) {
    setIndex(index);
  }
});


document.addEventListener('keydown', ev => {
  if (ev.key === 'PageDown') {
    ev.preventDefault();
    nextIndex();
  } else if (ev.key === 'PageUp') {
    ev.preventDefault();
    prevIndex();
  } else if (ev.key === 'Home') {
    ev.preventDefault();
    if (viewerOptions.value.zoom === 'contain') {
      setIndex(calculateIndex(0));
      return;
    }

    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.setObjectPosition(0, 0);
    // }
  } else if (ev.key === 'End') {
    ev.preventDefault();
    if (viewerOptions.value.zoom === 'contain') {
      setIndex(calculateIndex(currentFiles.value.length - 1));
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
      prevIndex();
      return;
    }
    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.moveMediaUp();
    // }
  } else if (ev.deltaY > 0) {
    if (viewerOptions.value.zoom === 'contain') {
      nextIndex();
      return;
    }
    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.moveMediaDown();
    // }
  }
};

</script>