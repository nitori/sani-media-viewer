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
    <div v-for="folder in folders" :ref="(elem) => focusPreviousFolder(folder, elem)">
      <a href="#" :class="[folder.path === previous?.path ? 'active' : '']"
         @click.prevent="$emit('select:folder', folder)">{{ folder.name }}</a>
    </div>
  </div>
</template>


<script setup lang="ts">
import type {FolderEntry} from "../types";
import {computed} from "vue";
import {focusElementInParent} from "../utils.ts";

const props = defineProps<{
  folders: FolderEntry[]
  previous: FolderEntry | null
  current: FolderEntry | null
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

const focusPreviousFolder = (folder: FolderEntry | null, elem: any) => {
  if (elem === null || folder === null) {
    return;
  }
  if (folder.path === props.previous?.path) {
    focusElementInParent(elem, elem.parentNode);
  }
};

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
