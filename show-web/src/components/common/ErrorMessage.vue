<script setup lang="ts">
defineProps<{
  message: string
  type?: 'error' | 'warning' | 'info'
}>()

defineEmits<{
  close: []
}>()
</script>

<template>
  <div class="error-container" :class="[`error-${type || 'error'}`]">
    <div class="error-content">
      <svg class="error-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <circle cx="12" cy="12" r="10"/>
        <line x1="12" y1="8" x2="12" y2="12"/>
        <line x1="12" y1="16" x2="12.01" y2="16"/>
      </svg>
      <span class="error-message">{{ message }}</span>
    </div>
    <button class="error-close" @click="$emit('close')">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <line x1="18" y1="6" x2="6" y2="18"/>
        <line x1="6" y1="6" x2="18" y2="18"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.error-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-radius: 12px;
  gap: 12px;
  animation: slideIn 0.3s ease;
}

.error-error {
  background: rgba(244, 63, 94, 0.1);
  border: 1px solid rgba(244, 63, 94, 0.3);
}

.error-warning {
  background: rgba(245, 158, 11, 0.1);
  border: 1px solid rgba(245, 158, 11, 0.3);
}

.error-info {
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
}

.error-content {
  display: flex;
  align-items: center;
  gap: 12px;
}

.error-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.error-error .error-icon {
  color: #f43f5e;
}

.error-warning .error-icon {
  color: #f59e0b;
}

.error-info .error-icon {
  color: #3b82f6;
}

.error-message {
  font-size: 14px;
  color: #fafafa;
}

.error-close {
  padding: 4px;
  border-radius: 6px;
  background: transparent;
  border: none;
  color: #a1a1aa;
  cursor: pointer;
  transition: all 0.2s;
  flex-shrink: 0;
}

.error-close:hover {
  background: rgba(255, 255, 255, 0.1);
  color: #fafafa;
}

.error-close svg {
  width: 16px;
  height: 16px;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
