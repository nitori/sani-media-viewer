<template>
  <div id="files">
    <div v-for="file in files" :ref="(elem) => focusCurrentFile(file, elem)">
      <a href="#" :class="[file.path === current?.path ? 'active' : '']"
         @click.prevent="$emit('select:media', file)">{{ file.name }}</a>
    </div>
  </div>
</template>

<script setup lang="ts">
import type {FileEntry} from "../types";
import {focusElementInParent} from "../utils.ts";

const props = defineProps<{
  files: FileEntry[],
  current: FileEntry | null
}>();

defineEmits<{
  (e: 'select:media', media: FileEntry): void
}>();

const focusCurrentFile = (file: FileEntry | null, elem: any) => {
  if (elem === null || file === null) {
    return;
  }
  if (file.path === props.current?.path) {
    focusElementInParent(elem, elem.parentNode);
  }
};

</script>
