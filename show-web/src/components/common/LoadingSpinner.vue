<script setup lang="ts">
defineProps<{
  size?: 'sm' | 'md' | 'lg'
  color?: string
}>()
</script>

<template>
  <div class="spinner-container" :class="[`spinner-${size || 'md'}`]">
    <div class="spinner">
      <div class="spinner-ring"></div>
      <div class="spinner-ring"></div>
      <div class="spinner-ring"></div>
    </div>
  </div>
</template>

<style scoped>
.spinner-container {
  display: inline-flex;
  justify-content: center;
  align-items: center;
}

.spinner-sm {
  width: 24px;
  height: 24px;
}

.spinner-md {
  width: 40px;
  height: 40px;
}

.spinner-lg {
  width: 64px;
  height: 64px;
}

.spinner {
  position: relative;
  width: 100%;
  height: 100%;
}

.spinner-ring {
  position: absolute;
  inset: 0;
  border-radius: 50%;
  border: 2px solid transparent;
  border-top-color: currentColor;
  animation: spin 1.2s cubic-bezier(0.5, 0, 0.5, 1) infinite;
}

.spinner-ring:nth-child(1) {
  animation-delay: -0.45s;
}

.spinner-ring:nth-child(2) {
  animation-delay: -0.3s;
}

.spinner-ring:nth-child(3) {
  animation-delay: -0.15s;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
