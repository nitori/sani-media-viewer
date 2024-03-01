<template>
  <div id="favs">
    <div v-for="fav in favs">
      <a href="#" :title="fav.name" @click.prevent="emitFolderChangeEvent(fav)">{{ fav.name }}</a>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, onMounted} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import type {FolderEntry} from "../types";

const favs = ref<{ name: string, path: string }[]>([]);

onMounted(async () => {
  favs.value = await invoke('get_favourites');
});

const emits = defineEmits<{
  (e: 'select:folder', folder: FolderEntry): void
}>();

function emitFolderChangeEvent(folder: { name: string, path: string }) {
  emits('select:folder', {
    name: folder.name,
    path: folder.path,
    symlink: false,
  });
}

</script>
