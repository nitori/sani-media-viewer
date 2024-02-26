<template>
  <div id="files">
    <div v-for="file in files">
      <a href="#" :class="[file.path === current?.path ? 'active' : '']"
         @click.prevent="$emit('select:media', file)">{{ file.name }}</a>
    </div>
  </div>
</template>

<script setup lang="ts">
import type {FileEntry, ViewerOptions} from "../types";

defineProps<{
  files: FileEntry[],
  current: FileEntry | null,
  options: ViewerOptions
}>();

defineEmits<
    (e: 'select:media', media: FileEntry) => void
>();

</script>
