<template>
  <div id="folders">
    <div v-for="folder in sortedFolders">
      <a href="#" :class="[folder.path === previous?.path ? 'active' : '']"
         @click.prevent="$emit('select:folder', folder)">{{ folder.name }}</a>
    </div>
  </div>
</template>


<script setup lang="ts">
import type {FolderEntry, ViewerOptions} from "../types";
import {computed} from "vue";

const props = defineProps<{
  folders: FolderEntry[],
  previous: FolderEntry | null
  options: ViewerOptions
}>();

defineEmits<
    (e: 'select:folder', folder: FolderEntry) => void
>();


const sortedFolders = computed(() => {
  let folders = [...props.folders].filter(f => {
    if (f.name != '..' && f.name.startsWith('.') && !props.options.showHidden) {
      return false;
    }
    return true;
  });

  return folders.sort((a, b) => {
    let aVal = a.name;
    let bVal = b.name;
    aVal = aVal.toLowerCase().replace(/[\[\]\(\)\{}<>.]+/g, '');
    bVal = bVal.toLowerCase().replace(/[\[\]\(\)\{}<>.]+/g, '');
    if (aVal === '..') return -1;
    if (bVal === '..') return 1;
    if (aVal === bVal) return 0;
    if (aVal < bVal) return -1;
    return 1;
  });
});

</script>
