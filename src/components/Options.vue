<template>
  <div id="shortcuts">
    <div class="icon" :class="[modelValue.sortBy === 'n' ? 'active' : '']" title="Sort by name: N"
         @click.prevent="$emit('update:modelValue',{...modelValue, sortBy:'n'})">
      <ArrowDownAZ v-if="!modelValue.sortReverse"/>
      <ArrowUpAZ v-else/>
    </div>
    <div class="icon" :class="[modelValue.sortBy === 'm' ? 'active' : '']" title="Sort by mtime: M"
         @click.prevent="$emit('update:modelValue',{...modelValue, sortBy: 'm'})">
      <ArrowDown19 v-if="!modelValue.sortReverse"/>
      <ArrowUp19 v-else/>
    </div>

    <div class="icon" :class="[modelValue.sortReverse ? 'active' : '']" title="Reverse sorting: R"
         @click.prevent="$emit('update:modelValue',{...modelValue, sortReverse: !modelValue.sortReverse})">
      <ArrowDownUp/>
    </div>

    <div class="icon active" v-if="modelValue.showHidden" title="Hide hidden files: H"
         @click.prevent="$emit('update:modelValue',{...modelValue, showHidden: false})">
      <EyeOpen/>
    </div>
    <div class="icon" v-if="!modelValue.showHidden" title="Show hidden files: H"
         @click.prevent="$emit('update:modelValue',{...modelValue, showHidden: true})">
      <EyeSlash/>
    </div>

    <div class="icon" title="Toggle full screen: F" :class="[modelValue.fullScreen ? 'active' : '']"
         @click.prevent="$emit('update:modelValue',{...modelValue, fullScreen: !modelValue.fullScreen})">
      <Expand/>
    </div>

    <div class="icon" title="Zoom to cover: z"
         v-if="modelValue.zoom === 'contain'"
         @click.prevent="$emit('update:modelValue',{...modelValue, zoom: 'cover'})">
      <MagnPlus/>
    </div>
    <div class="icon active" title="Zoom to original size: z"
         v-if="modelValue.zoom === 'cover'"
         @click.prevent="$emit('update:modelValue',{...modelValue, zoom: 'none'})">
      <MagnMinus/>
    </div>
    <div class="icon active" title="Reset to fit: z"
         v-if="modelValue.zoom === 'none'"
         @click.prevent="$emit('update:modelValue',{...modelValue, zoom: 'contain'})">
      <Magn/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, watch} from "vue";
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
  modelValue: ViewerOptions
}>();

defineEmits<{
  (e: 'update:modelValue', value: ViewerOptions): void,
}>();

</script>


<style scoped>
svg path {
  fill: currentColor;
}

#shortcuts {
  display: flex;
  align-items: center;
  color: white;
  font-size: 36px;
  height: 40px;
}

#shortcuts .icon {
  flex: 0 1 1em;
  max-width: 1em;
  width: 1em;
  line-height: 14px;
  padding: 5px;
  cursor: pointer;
}

#shortcuts svg {
  display: block;
  width: 100%;
  height: auto;
}

</style>
