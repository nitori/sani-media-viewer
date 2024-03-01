<template>
  <div id="shortcuts">
    <div class="icon" :class="[options.sortBy === 'n' ? 'active' : '']" title="Sort by name: N"
         @click.prevent="$emit('option:sortBy', 'n')">
      <ArrowDownAZ v-if="!options.sortReverse"/>
      <ArrowUpAZ v-else/>
    </div>
    <div class="icon" :class="[options.sortBy === 'm' ? 'active' : '']" title="Sort by mtime: M"
         @click.prevent="$emit('option:sortBy', 'm')">
      <ArrowDown19 v-if="!options.sortReverse"/>
      <ArrowUp19 v-else/>
    </div>

    <div class="icon" :class="[options.sortReverse ? 'active' : '']" title="Reverse sorting: R"
         @click.prevent="$emit('option:sortReverse', !options.sortReverse)">
      <ArrowDownUp/>
    </div>

    <div class="icon active" v-if="options.showHidden" title="Hide hidden files: H"
         @click.prevent="$emit('option:showHidden', false)">
      <EyeOpen/>
    </div>
    <div class="icon" v-if="!options.showHidden" title="Show hidden files: H"
         @click.prevent="$emit('option:showHidden', true)">
      <EyeSlash/>
    </div>

    <div class="icon" title="Toggle full screen: F" :class="[options.fullScreen ? 'active' : '']"
         @click.prevent="$emit('option:fullScreen', !options.fullScreen)">
      <Expand/>
    </div>

    <div class="icon" title="Zoom to cover: z"
         v-if="options.zoom === 'contain'"
         @click.prevent="$emit('option:zoom', 'cover')">
      <MagnPlus/>
    </div>
    <div class="icon active" title="Zoom to original size: z"
         v-if="options.zoom === 'cover'"
         @click.prevent="$emit('option:zoom', 'none')">
      <MagnMinus/>
    </div>
    <div class="icon active" title="Reset to fit: z"
         v-if="options.zoom === 'none'"
         @click.prevent="$emit('option:zoom', 'contain')">
      <Magn/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ViewerOptions} from "../types";
import ArrowDown19 from "../assets/icons/arrow-down-1-9-sharp-solid.svg";
import ArrowDownAZ from "../assets/icons/arrow-down-a-z-sharp-solid.svg";
import ArrowUp19 from "../assets/icons/arrow-up-1-9-sharp-solid.svg";
import ArrowUpAZ from "../assets/icons/arrow-up-a-z-sharp-solid.svg";
import ArrowDownUp from "../assets/icons/arrow-down-arrow-up-sharp-solid.svg";
import Expand from "../assets/icons/expand-sharp-solid.svg";
import EyeOpen from "../assets/icons/eye-sharp-solid.svg";
import EyeSlash from "../assets/icons/eye-slash-sharp-solid.svg";
import MagnMinus from "../assets/icons/magnifying-glass-minus-sharp-solid.svg";
import MagnPlus from "../assets/icons/magnifying-glass-plus-sharp-solid.svg";
import Magn from "../assets/icons/magnifying-glass-sharp-solid.svg";

defineProps<{
  options: ViewerOptions
}>();

defineEmits<{
  (e: 'option:sortBy', value: 'm' | 'n'): void,
  (e: 'option:sortReverse', value: boolean): void,
  (e: 'option:showHidden', value: boolean): void,
  (e: 'option:fullScreen', value: boolean): void,
  (e: 'option:zoom', value: 'cover' | 'contain' | 'none'): void,
}>();

</script>
