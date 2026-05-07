<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useMapStore } from './stores/map'
import { useDisplayStore } from './stores/display'

const mapStore = useMapStore()
const displayStore = useDisplayStore()

onMounted(() => {
  mapStore.loadMaps()
  displayStore.loadDisplays()
})
</script>

<style scoped>
#app {
  min-height: 100vh;
  background: linear-gradient(135deg, #0f1419 0%, #1a1f26 50%, #0f1419 100%);
}
</style>
