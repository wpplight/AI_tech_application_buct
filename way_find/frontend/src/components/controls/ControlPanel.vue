<template>
  <div class="control-panel">
    <h3 class="title">Controls</h3>
    
    <div class="button-group">
      <button 
        class="control-btn primary"
        :disabled="animation.isRunning"
        @click="handlePlay"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
          <path d="M8 5v14l11-7z"/>
        </svg>
        <span>{{ animation.isRunning ? 'Running...' : 'Start' }}</span>
      </button>
      
      <button 
        class="control-btn secondary"
        :disabled="!animation.isRunning || animation.isPaused"
        @click="handlePause"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 19h4V5H6v14zm8-14v14h4V5h-4z"/>
        </svg>
        <span>Pause</span>
      </button>
      
      <button 
        class="control-btn danger"
        :disabled="!animation.isRunning && !animation.isPaused"
        @click="handleStop"
      >
        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
          <path d="M6 6h12v12H6z"/>
        </svg>
        <span>Stop</span>
      </button>
    </div>
    
    <div class="divider"></div>
    
    <button 
      class="control-btn reset"
      @click="handleReset"
    >
      <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
        <path d="M17.65 6.35A7.958 7.958 0 0012 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0112 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z" fill="currentColor"/>
      </svg>
      <span>Reset</span>
    </button>
    
    <div class="speed-control">
      <label class="speed-label">Speed: {{ speed }}x</label>
      <input 
        type="range" 
        min="1" 
        max="10" 
        v-model.number="speed"
        class="speed-slider"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useMazeStore } from '@/stores/maze'

const store = useMazeStore()
const animation = computed(() => store.animation)

const speed = ref(1)

watch(speed, (newSpeed) => {
  store.setSpeed(newSpeed)
})

function handlePlay() {
  store.play()
}

function handlePause() {
  store.pause()
}

function handleStop() {
  store.stop()
}

function handleReset() {
  store.reset()
}
</script>

<style scoped>
.control-panel {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.title {
  font-size: 1.125rem;
  font-weight: 600;
  color: white;
  margin-bottom: 0.5rem;
}

.button-group {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.control-btn {
  flex: 1;
  min-width: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border-radius: 12px;
  border: none;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: inherit;
}

.control-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none !important;
}

.control-btn:not(:disabled):hover {
  transform: translateY(-2px);
}

.control-btn:not(:disabled):active {
  transform: translateY(0) scale(0.98);
}

.control-btn.primary {
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.3);
}

.control-btn.primary:hover:not(:disabled) {
  box-shadow: 0 6px 20px rgba(16, 185, 129, 0.4);
}

.control-btn.secondary {
  background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.control-btn.secondary:hover:not(:disabled) {
  box-shadow: 0 6px 20px rgba(59, 130, 246, 0.4);
}

.control-btn.danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
}

.control-btn.danger:hover:not(:disabled) {
  box-shadow: 0 6px 20px rgba(239, 68, 68, 0.4);
}

.control-btn.reset {
  background: rgba(255, 255, 255, 0.05);
  color: rgba(255, 255, 255, 0.8);
  border: 1px solid rgba(255, 255, 255, 0.1);
  backdrop-filter: blur(10px);
}

.control-btn.reset:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.2);
  color: white;
}

.divider {
  height: 1px;
  background: rgba(255, 255, 255, 0.1);
  margin: 0.5rem 0;
}

.speed-control {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.speed-label {
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.6);
}

.speed-slider {
  width: 100%;
  height: 6px;
  border-radius: 3px;
  background: rgba(255, 255, 255, 0.1);
  outline: none;
  -webkit-appearance: none;
}

.speed-slider::-webkit-slider-thumb {
  -webkit-appearance: none;
  width: 18px;
  height: 18px;
  border-radius: 50%;
  background: linear-gradient(135deg, #10b981 0%, #059669 100%);
  cursor: pointer;
  box-shadow: 0 2px 8px rgba(16, 185, 129, 0.3);
  transition: all 0.2s ease;
}

.speed-slider::-webkit-slider-thumb:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(16, 185, 129, 0.4);
}
</style>
