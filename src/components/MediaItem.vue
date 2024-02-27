<template>
  <img v-if="media && !isVideo(media.name)" class="media-item" :src="mediaSrc" :alt="media.name"/>
  <video v-if="media && isVideo(media.name)" class="media-item" :src="mediaSrc" controls autoplay></video>
</template>

<script setup lang="ts">
import type {FileEntry} from "../types";
import {ref, watch} from "vue";
import {convertFileSrc} from '@tauri-apps/api/tauri';
import {isVideo} from "../utils.ts";

const props = defineProps<{
  media: FileEntry | null
}>()

const mediaSrc = ref<string>('');

watch(() => props.media, async (newMedia) => {
  if (newMedia) {
    mediaSrc.value = convertFileSrc(newMedia.path);
  }
});

</script>
