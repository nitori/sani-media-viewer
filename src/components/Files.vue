<template>
  <div id="files">
    <div v-for="file in sortedFiles">
      <a href="#" :class="[file.path === current?.path ? 'active' : '']"
         @click.prevent="$emit('select:media', file)">{{ file.name }}</a>
    </div>
  </div>
</template>

<script setup lang="ts">
import type {FileEntry, ViewerOptions} from "../types";
import {computed} from "vue";

const props = defineProps<{
  files: FileEntry[],
  current: FileEntry | null,
  options: ViewerOptions
}>();

defineEmits<
    (e: 'select:media', media: FileEntry) => void
>();


const sortedFiles = computed(() => {
  let files = [...props.files].filter(f => {
    if (f.name.startsWith('.') && !props.options.showHidden) {
      return false;
    }
    return true;
  });

  const getVal = (file: FileEntry) => {
    if (props.options.sortBy === 'm') {
      return file.mtime;
    }
    return file.name.toLowerCase().replace(/[\[\]\(\)\{}<>.]+/g, '');
  }

  return files.sort((a, b) => {
    let aVal = getVal(a);
    let bVal = getVal(b);
    if (aVal === bVal) {
      return 0;
    }
    if (aVal < bVal) {
      return props.options.sortReverse ? 1 : -1;
    }
    return props.options.sortReverse ? -1 : 1;
  })
});

</script>
