<template>
  <img v-if="media && isImage" class="media-item" :src="mediaSrc" :alt="media.name"/>
  <video v-if="media && isVideo" class="media-item" :src="mediaSrc" controls autoplay></video>
</template>

<script setup lang="ts">
import type {FileEntry} from "../types";
import {computed, ref, watch} from "vue";
import {convertFileSrc} from '@tauri-apps/api/tauri';

const props = defineProps<{
  media: FileEntry | null
}>()

const mediaSrc = ref<string>('');

watch(() => props.media, async (newMedia) => {
  if (newMedia) {
    mediaSrc.value = convertFileSrc(newMedia.path);
  }
});

const _isVideo = (name: string) => {
  return name.endsWith(".mp4") || name.endsWith(".webm");
};

const isImage = computed(() => {
  return props.media && !_isVideo(props.media.name);
});

const isVideo = computed(() => {
  return props.media && _isVideo(props.media.name);
});

</script>
