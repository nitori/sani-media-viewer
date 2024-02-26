<template>
  <div id="folders">
    <div v-for="folder in folders">
      <a href="#" :class="[folder.path === previous?.path ? 'active' : '']"
         @click.prevent="$emit('select:folder', folder)">{{ folder.name }}</a>
    </div>
  </div>
</template>


<script setup lang="ts">
import type {FolderEntry, ViewerOptions} from "../types";

defineProps<{
  folders: FolderEntry[],
  previous: FolderEntry | null
  options: ViewerOptions
}>();

defineEmits<
    (e: 'select:folder', folder: FolderEntry) => void
>();

</script>
