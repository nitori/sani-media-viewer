<template>
  <div id="main">
    <div id="left">
      <div id="path">{{ folderListing.canonical_path }}</div>
      <div id="shortcuts-meta">
        <Options v-model="viewerOptions"/>
        <div id="meta"></div>
      </div>
      <div id="favs-folders">
        <Favourites @select:folder="currentFolder = $event"/>
        <Folders :previous="previousFolder"
                 :folders="folderListing.folders"
                 :options="viewerOptions"
                 @select:folder="currentFolder = $event"/>
      </div>
      <Files :current="currentMedia"
             :files="folderListing.files"
             :options="viewerOptions"
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
import type {FolderList, ViewerOptions, FolderEntry, FileEntry} from "./types";
import {invoke} from "@tauri-apps/api/tauri";
import MediaItem from "./components/MediaItem.vue";
import Options from "./components/Options.vue";

const folderListing = ref<FolderList>({
  canonical_path: "",
  folders: [],
  files: [],
  hash: "",
});

const currentFolder = ref<FolderEntry | null>(null);
const previousFolder = ref<FolderEntry | null>(null);
const currentMedia = ref<FileEntry | null>(null);
const currentMediaIndex = ref<number>(-1);
const imageContainer = ref<HTMLDivElement | null>(null);

const viewerOptions = ref<ViewerOptions>({
  sortBy: "n",
  sortReverse: false,
  showHidden: false,
  fullScreen: false,
  zoom: 'contain',
});

watch(currentFolder, (newFolder, oldFolder) => {
  if (oldFolder) {
    previousFolder.value = oldFolder;
  }

  if (newFolder) {
    (async () => {
      folderListing.value = await invoke("get_list", {path: newFolder.path});
      setIndex(0);
    })();
  }
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

onMounted(async () => {
  folderListing.value = await invoke("get_list", {path: ""});
  setIndex(0);
});

const onFullscreenChange = (_ev: Event) => {
  if (document.fullscreenElement === null && viewerOptions.value.fullScreen) {
    viewerOptions.value.fullScreen = false;
  }
};

const setIndex = (index: number) => {
  index = Math.min(folderListing.value.files.length - 1, Math.max(0, index));
  currentMediaIndex.value = index;
  if (index < 0) {
    currentMedia.value = null;
  } else {
    currentMedia.value = folderListing.value.files[index];
  }
};

const nextIndex = () => {
  setIndex(currentMediaIndex.value + 1);
};

const prevIndex = () => {
  setIndex(currentMediaIndex.value - 1);
};


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
      setIndex(0);
      return;
    }

    // if (this._isObjectFitCover() || this._isObjectFitNone()) {
    //   this.setObjectPosition(0, 0);
    // }
  } else if (ev.key === 'End') {
    ev.preventDefault();
    if (viewerOptions.value.zoom === 'contain') {
      setIndex(folderListing.value.files.length - 1);
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