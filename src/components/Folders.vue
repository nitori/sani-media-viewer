<template>
  <div id="folders">
    <div class="path" :title="current?.path">
      <div class="path-dir">
        {{ dirname }}
      </div>
      <div class="path-base">
        /{{ basename }}
      </div>
    </div>
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
  current: FolderEntry | null
  options: ViewerOptions
}>();

defineEmits<
    (e: 'select:folder', folder: FolderEntry) => void
>();

const dirname = computed(() => {
  if (props.current) {
    return props.current.path.split('/').slice(0, -1).join('/');
  }
  return '';
});

const basename = computed(() => {
  if (props.current) {
    return props.current.path.split('/').pop();
  }
  return '';
});

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

<style scoped>
.path {
  display: flex;
  white-space: nowrap;
  text-align: left;
}

.path-dir {
  flex: 0 1 auto;
  overflow: hidden;
  text-overflow: ellipsis;
}

.path-base {
  flex: 1 0 max-content;
  max-width: max-content;
}
</style>
